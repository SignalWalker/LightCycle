use super::super::{plane::polygon::*, *};
use na::{Matrix3, Point3, Scalar};
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

impl Polyhedron<f32> {
    pub fn cube() -> Polyhedron<f32> {
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

    pub fn octohedron() -> Polyhedron<f32> {
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

    pub fn triangle() -> Polyhedron<f32> {
        Polyhedron {
            points: vec![
                [1.0, -1.0, 0.0].into(),
                [0.0, 1.0, 0.0].into(),
                [-1.0, -1.0, 0.0].into(),
            ],
            faces: vec![[0, 1, 2]],
        }
    }

    pub fn plane() -> Polyhedron<f32> {
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

    pub fn inertia_moment(&self) -> Matrix3<f32> {
        Matrix3::identity()
    }
}

pub fn face_2d(face: [Point3<f32>; 3]) -> Polygon<f32> {
    Polygon {
        points: vec![
            Point2::new(face[0][0], face[0][1]),
            Point2::new(face[1][0], face[1][1]),
            Point2::new(face[2][0], face[2][1]),
        ],
    }
}

pub fn tri_center(face: &[Point3<f32>; 3]) -> Point3<f32> {
    Point3::from((face[0].coords + face[1].coords + face[2].coords) / 3.0)
}
