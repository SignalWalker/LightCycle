use super::{camera::*, *};
use na::{Isometry3, Matrix4, Point3, Scalar, Vector2};
use plane::*;
use std::fmt::{Debug, Display, Formatter};
use std::ops::*;

pub mod polyhedron;

pub type Line3<N: Scalar> = [Point3<N>; 2];

pub fn line_intersection(a: &Line3<f64>, b: &Line3<f64>) -> Option<Point3<f64>> {
    let delta_a = a[1] - a[0];
    let delta_b = b[1] - b[0];
    panic!("Not Implemented: 3D Line Intersection")
}

pub fn ndc_to_screen(ndc: &Point3<f64>, size: &Point2<f64>, clip: &Vector2<f64>) -> Point2<f64> {
    let res = Point2::from_homogeneous(ndc.coords).unwrap();
    Point2::new(
        (res[0] + 1.0) * (size[0] / 2.0) + clip[0],
        (res[1] + 1.0) * (size[1] / 2.0) + clip[1],
    )
}

pub fn draw_line(
    buf: &mut Buffer<Color>,
    c: Color,
    l: Line3<f64>,
    model_world: Matrix4<f64>,
    camera: &Camera,
) {
    let mat = camera.iso().to_homogeneous() * model_world;
    let size = Point2::new(buf.size[0] as f64, buf.size[1] as f64);
    let p1 = mat * l[0].to_homogeneous();
    let p2 = mat * l[1].to_homogeneous();
    let s1 = ndc_to_screen(
        &Point3::from_homogeneous(p1).unwrap(),
        &size,
        &l[0].coords.xy(),
    );
    let s2 = ndc_to_screen(
        &Point3::from_homogeneous(p2).unwrap(),
        &size,
        &l[1].coords.xy(),
    );
    let line = Some(([s1, s2], ())); //plane::lb_clip(&[s1, s2], &[Point2::new(0.0, 0.0), size]);
    let line = match line {
        Some((l, ..)) => [
            Point2::new(l[0].x as isize, l[0].y as isize),
            Point2::new(l[1].x as isize, l[1].y as isize),
        ],
        None => return,
    };
    println!("Drawing Norm: {:?}", line);
    plane::draw_line(buf, c, line)
}

//use polyhedron::*;

// #[derive(Copy, Clone, Debug, PartialEq, Eq)]
// pub struct Point3<N>(pub [N; 3]);

// impl<N: Display> Display for Point3<N> {
//     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
//         write!(f, "[{}, {}, {}]", self.0[0], self.0[1], self.0[2])
//     }
// }

// impl<L, R, O> Add<Point3<R>> for Point3<L>
// where
//     L: Add<R, Output = O> + Copy,
//     R: Copy,
// {
//     type Output = Point3<O>;
//     fn add(self, r: Point3<R>) -> Self::Output {
//         Point3([self.0[0] + r.0[0], self.0[1] + r.0[1], self.0[2] + r.0[2]])
//     }
// }

// impl<L, R, O> Sub<Point3<R>> for Point3<L>
// where
//     L: Sub<R, Output = O> + Copy,
//     R: Copy,
// {
//     type Output = Point3<O>;
//     fn sub(self, r: Point3<R>) -> Self::Output {
//         Point3([self.0[0] - r.0[0], self.0[1] - r.0[1], self.0[2] - r.0[2]])
//     }
// }

// impl<L, R, O> Mul<R> for Point3<L>
// where
//     L: Mul<R, Output = O> + Copy,
//     R: Copy,
// {
//     type Output = Point3<O>;
//     fn mul(self, r: R) -> Self::Output {
//         Point3([self.0[0] * r, self.0[1] * r, self.0[2] * r])
//     }
// }

// impl<L, R, O> Div<R> for Point3<L>
// where
//     L: Div<R, Output = O> + Copy,
//     R: Copy,
// {
//     type Output = Point3<O>;
//     fn div(self, r: R) -> Self::Output {
//         Point3([self.0[0] / r, self.0[1] / r, self.0[2] / r])
//     }
// }

// impl<L: Neg + Copy> Neg for Point3<L> {
//     type Output = Point3<L::Output>;
//     fn neg(self) -> Self::Output {
//         Point3([-self.0[0], -self.0[1], -self.0[2]])
//     }
// }

// impl<N> Index<usize> for Point3<N> {
//     type Output = N;
//     fn index(&self, index: usize) -> &N {
//         &self.0[index]
//     }
// }

// impl<N> IndexMut<usize> for Point3<N> {
//     fn index_mut(&mut self, index: usize) -> &mut N {
//         &mut self.0[index]
//     }
// }

// impl<N> From<[N; 3]> for Point3<N> {
//     fn from(arr: [N; 3]) -> Point3<N> {
//         Point3(arr)
//     }
// }

// impl<N> From<(N, N, N)> for Point3<N> {
//     fn from(arr: (N, N, N)) -> Point3<N> {
//         Point3([arr.0, arr.1, arr.2])
//     }
// }

// impl<N> Point3<N> {
//     pub fn xy(self) -> Point2<N>
//     where
//         N: Copy,
//     {
//         Point2([self.0[0], self.0[1]])
//     }

//     pub fn xz(self) -> Point2<N>
//     where
//         N: Copy,
//     {
//         Point2([self.0[0], self.0[2]])
//     }

//     pub fn yz(self) -> Point2<N>
//     where
//         N: Copy,
//     {
//         Point2([self.0[1], self.0[2]])
//     }

//     pub fn dot<R, M, O>(self, r: Point3<R>) -> O
//     where
//         N: Mul<R, Output = M> + Copy,
//         R: Copy,
//         M: Add<M, Output = O> + Add<O, Output = O>,
//         O: Add<O, Output = O> + Add<M, Output = O>,
//     {
//         self.0[0] * r.0[0] + self.0[1] * r.0[1] + self.0[2] * r.0[2]
//     }

//     pub fn cross<R, M, O>(self, r: Point3<R>) -> Point3<O>
//     where
//         N: Mul<R, Output = M> + Copy,
//         R: Copy,
//         M: Sub<M, Output = O>,
//     {
//         Point3([
//             self.0[1] * r.0[2] - self.0[2] * r.0[1],
//             self.0[2] * r.0[0] - self.0[0] * r.0[2],
//             self.0[0] * r.0[1] - self.0[1] * r.0[0],
//         ])
//     }

//     pub fn outcode<R>(self, prism: Point2<Point3<R>>) -> u8
//     where
//         N: PartialOrd<R>,
//     {
//         const UP: u8 = 0b100000;
//         const DOWN: u8 = 0b010000;
//         const LEFT: u8 = 0b001000;
//         const RIGHT: u8 = 0b000100;
//         const IN: u8 = 0b000010;
//         const OUT: u8 = 0b000001;

//         let mut res = 0x000000;
//         if self[0] < prism[0][0] {
//             res &= LEFT
//         } else if self[0] > prism[1][0] {
//             res &= RIGHT
//         }

//         if self[1] < prism[0][1] {
//             res &= DOWN
//         } else if self[1] > prism[1][1] {
//             res &= UP
//         }

//         if self[2] < prism[0][2] {
//             res &= IN
//         } else if self[2] > prism[1][2] {
//             res &= OUT
//         }
//         res
//     }
// }

// impl Point3<f64> {
//     pub fn mag(self) -> f64 {
//         (self.0[0].powi(2) + self.0[1].powi(2) + self.0[2].powi(2)).sqrt()
//     }
// }

// impl Point2<Point3<f64>> {
//     pub fn line_plane_intersection(self, plane: Self) -> Option<Point3<f64>> {
//         let u = self.delta();
//         let dot = plane.0[1].dot(u);
//         if dot.abs() > 0.0 {
//             let w = self.0[0] - plane.0[0];
//             let fac = -(plane.0[1].dot(w)) / dot;
//             return Some(self.0[0] + (u * fac));
//         }
//         None
//     }

//     pub fn lb_clip(mut self, mut bound: Self, epsilon: f64) -> Option<(Self, [bool; 2])> {
//         fn clip(p: f64, q: f64, mut t: [f64; 2], epsilon: f64) -> Option<[f64; 2]> {
//             println!("P: {} Q: {} T: {:?}", p, q, t);
//             if p.abs() < epsilon && q < 0.0 {
//                 return None;
//             }
//             let r = q / p;
//             //println!("R: {}", r);
//             if p < 0.0 {
//                 if r > t[1] {
//                     return None;
//                 }
//                 if r > t[0] {
//                     t[0] = r
//                 }
//             } else {
//                 if r < t[0] {
//                     return None;
//                 }
//                 if r < t[1] {
//                     t[1] = r
//                 }
//             }
//             Some(t)
//         }

//         println!("Clipping {} with {}", self, bound);

//         // making sure the line defining the boundary is positive
//         bound = bound.reorder_prism();

//         let delta = self.delta();

//         let clip = clip(
//             delta[2],
//             bound[1][2] - self[0][2],
//             clip(
//                 -delta[2],
//                 -(bound[0][2] - self[0][2]),
//                 clip(
//                     delta[1],
//                     bound[1][1] - self[0][1],
//                     clip(
//                         -delta[1],
//                         -(bound[0][1] - self[0][1]),
//                         clip(
//                             delta[0],
//                             bound[1][0] - self[0][0],
//                             clip(-delta[0], -(bound[0][0] - self[0][0]), [0.0, 1.0], epsilon)?,
//                             epsilon,
//                         )?,
//                         epsilon,
//                     )?,
//                     epsilon,
//                 )?,
//                 epsilon,
//             )?,
//             epsilon,
//         )?;

//         let mut clipped = [false, false];
//         if clip[0].abs() < epsilon {
//             clipped[0] = true;
//             self[0][0] = self[0][0] + clip[0] * delta[0];
//             self[0][1] = self[0][1] + clip[0] * delta[1];
//             self[0][2] = self[0][2] + clip[0] * delta[2];
//         }
//         if clip[1].abs() < epsilon {
//             clipped[1] = true;
//             self[1][0] = self[0][0] + clip[1] * delta[0];
//             self[1][1] = self[0][1] + clip[1] * delta[1];
//             self[1][2] = self[0][2] + clip[1] * delta[2];
//         }

//         println!("Clipped: {}", self);

//         Some((self, clipped))
//     }
// }

// impl<N> Point2<Point3<N>> {
//     /// Changes order of prism values to make all deltas positive
//     pub fn reorder_prism(mut self) -> Self
//     where
//         N: PartialOrd<N> + Copy,
//     {
//         if self[0][0] > self[1][0] {
//             let temp = self[1][0];
//             self[1][0] = self[0][0];
//             self[0][0] = temp;
//         }
//         if self[0][1] > self[1][1] {
//             let temp = self[1][1];
//             self[1][1] = self[0][1];
//             self[0][1] = temp;
//         }
//         if self[0][2] > self[1][2] {
//             let temp = self[1][2];
//             self[1][2] = self[0][2];
//             self[0][2] = temp;
//         }
//         self
//     }
// }
