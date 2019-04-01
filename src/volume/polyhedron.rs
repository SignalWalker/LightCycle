use super::super::{camera::*, plane::polygon::*, *};
//use super::matrix::*;
use super::*;
use na::{Matrix3, Matrix4, Point3, Scalar, Vector3, Vector4};
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug)]
pub struct Polyhedron<N: Scalar> {
    pub points: Vec<Point3<N>>,
    pub faces: Vec<[usize; 3]>,
}

impl<N: Scalar> Index<usize> for Polyhedron<N> {
    type Output = Point3<N>;
    fn index(&self, i: usize) -> &Point3<N> {
        &self.points[i]
    }
}

impl<N: Scalar> IndexMut<usize> for Polyhedron<N> {
    fn index_mut(&mut self, i: usize) -> &mut Point3<N> {
        &mut self.points[i]
    }
}

impl Polyhedron<f64> {
    pub fn cube() -> Polyhedron<f64> {
        Polyhedron {
            points: vec![
                // back
                [1.0, -1.0, -1.0].into(),
                [-1.0, -1.0, -1.0].into(),
                [-1.0, 1.0, -1.0].into(),
                [1.0, 1.0, -1.0].into(),
                // front
                [-1.0, -1.0, 1.0].into(),
                [1.0, -1.0, 1.0].into(),
                [1.0, 1.0, 1.0].into(),
                [-1.0, 1.0, 1.0].into(),
            ],
            faces: vec![
                // back
                [0, 1, 2],
                [2, 3, 0],
                // front
                [4, 5, 6],
                [6, 7, 4],
                // bottom
                [5, 4, 1],
                [1, 0, 5],
                // top
                [3, 2, 7],
                [7, 6, 3],
                // left
                [1, 4, 7],
                [7, 2, 1],
                // right
                [5, 0, 3],
                [3, 6, 5],
            ],
        }
    }

    pub fn octohedron() -> Polyhedron<f64> {
        Polyhedron {
            points: vec![
                // center
                [-1.0, 0.0, -1.0].into(),
                [1.0, 0.0, -1.0].into(),
                [1.0, 0.0, 1.0].into(),
                [-1.0, 0.0, 1.0].into(),
                // top
                [0.0, 2.0, 0.0].into(),
                // bottom
                [0.0, -2.0, 0.0].into(),
            ],
            faces: vec![
                // top
                [4, 1, 0],
                [4, 2, 1],
                [4, 3, 2],
                [4, 0, 3],
                // bottom
                [5, 0, 1],
                [5, 1, 2],
                [5, 2, 3],
                [5, 3, 0],
            ],
        }
    }

    pub fn triangle() -> Polyhedron<f64> {
        Polyhedron {
            points: vec![
                [1.0, -1.0, 0.0].into(),
                [0.0, 1.0, 0.0].into(),
                [-1.0, -1.0, 0.0].into(),
            ],
            faces: vec![[0, 1, 2]],
        }
    }

    pub fn plane() -> Polyhedron<f64> {
        Polyhedron {
            points: vec![
                [1.0, 0.0, -1.0].into(),
                [-1.0, 0.0, -1.0].into(),
                [-1.0, 0.0, 1.0].into(),
                [1.0, 0.0, 1.0].into(),
            ],
            faces: vec![[0, 1, 2], [2, 3, 0]],
        }
    }

    pub fn inertia_moment(&self) -> Matrix3<f64> {
        Matrix3::identity()
    }

    pub fn draw(&self, buf: &mut Buffer<Color>, model_world: Matrix4<f64>, camera: &Camera) {
        let width = buf.size[0] as f64;
        let height = buf.size[1] as f64;
        let model_view = camera.cache * model_world;
        let transform = camera.persp.to_homogeneous() * model_view;
        for i in 0..self.faces.len() {
            let face = &self.faces[i];

            // println!(
            //     "Face {}: [\n{}\n{}\n{}\n]",
            //     i, self[face[0]], self[face[1]], self[face[2]]
            // );

            let v1 = Vector4::new(self[face[0]][0], self[face[0]][1], self[face[0]][2], 1.0);
            let v2 = Vector4::new(self[face[1]][0], self[face[1]][1], self[face[1]][2], 1.0);
            let v3 = Vector4::new(self[face[2]][0], self[face[2]][1], self[face[2]][2], 1.0);

            let norm: Vector3<f64> = Vector3::from_homogeneous(
                model_world
                    * (self[face[1]] - self[face[0]])
                        .cross(&(self[face[2]] - self[face[1]]))
                        .to_homogeneous(),
            )
            .unwrap();
            if norm.dot(&(camera.pos - self[face[0]])) < 0.0 {
                //if camera.forward.dot(&norm) > 0.0 && norm.dot(&camera.forward) < 0.0 {
                continue;
            }

            let t1 = transform * v1;
            let t2 = transform * v2;
            let t3 = transform * v3;

            //println!("Transformed: [\n{}\n{}\n{}\n]", t1, t2, t3);

            let h1 = match Point3::from_homogeneous(t1) {
                Some(p) => p,
                None => continue,
            };
            let h2 = match Point3::from_homogeneous(t2) {
                Some(p) => p,
                None => continue,
            };
            let h3 = match Point3::from_homogeneous(t3) {
                Some(p) => p,
                None => continue,
            };

            //println!("H1: {}, H2: {}, H3: {}", h1, h2, h3);

            fn clipped(p: &Point3<f64>) -> bool {
                false
                    && (p.x > 1.0
                        || p.x < -1.0
                        || p.y > 1.0
                        || p.y < -1.0
                        || p.z > 1.0
                        || p.z < -1.0)
            }

            if clipped(&h1) || clipped(&h2) || clipped(&h3) {
                return;
            }

            //println!("Homo: [\n{}\n{}\n{}\n]", h1, h2, h3);

            let size = Point2::new(width, height);
            let p1 = ndc_to_screen(&h1, &size, &t1.xy());
            let p2 = ndc_to_screen(&h2, &size, &t2.xy());
            let p3 = ndc_to_screen(&h3, &size, &t3.xy());
            //let l1 = [p1, p2];
            //let l2 = [p2, p3];
            //let mut norm = camera.iso.to_homogeneous() * normalize(&(p2 - p1).cross(&(p3 - p2)));
            //if norm[2] < 0.0 {
            //println!("Skipping norm: {}", norm);
            //continue;
            //}
            let color = [
                (2 * norm[0].abs() as usize % 255) as u8,
                (2 * norm[1].abs() as usize % 255) as u8,
                (2 * norm[2].abs() as usize % 255) as u8,
                255,
            ];
            //let d_norm = Matrix4::new_nonuniform_scaling(&Vector3::new(width, height, 1.0)) * norm;
            //println!("Drawing Norm: {}", d_norm);
            // draw_line(
            //     buf,
            //     color,
            //     [
            //         Point2::new(width as isize / 2, height as isize / 2),
            //         Point2::new(d_norm[0] as isize, d_norm[1] as isize),
            //     ],
            // );
            //println!("Triangle: [\n{}\n{}\n{}\n]", p1, p2, p3);
            let mut poly = Polygon {
                points: vec![p1, p2, p3],
            };
            if let Some(p) = poly.clip(&[
                Point2::new(1.0, 1.0),
                Point2::new(buf.size[0] as f64 - 1.0, buf.size[1] as f64 - 1.0),
            ]) {
                p.draw(buf, color);
                // let center = tri_center(&[self[face[0]], self[face[1]], self[face[2]]]);
                // //println!("Norm: [{}, {}]", center, center + norm);
                // volume::draw_line(
                //     buf,
                //     [255, 255, 255, 255],
                //     [center, center + norm],
                //     model_world,
                //     &camera,
                // )
            }
        }
    }
}

pub fn face_2d(face: [Point3<f64>; 3]) -> Polygon<f64> {
    Polygon {
        points: vec![
            Point2::new(face[0][0], face[0][1]),
            Point2::new(face[1][0], face[1][1]),
            Point2::new(face[2][0], face[2][1]),
        ],
    }
}

pub fn tri_center(face: &[Point3<f64>; 3]) -> Point3<f64> {
    Point3::from((face[0].coords + face[1].coords + face[2].coords) / 3.0)
}
