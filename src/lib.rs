#[macro_use]
pub extern crate approx;
pub extern crate nalgebra as na;

use na::Point2;
use std::ops::{Index, IndexMut};

pub mod camera;
pub mod plane;
pub mod volume;

#[derive(Copy, Clone, Debug)]
pub enum Either<A, B> {
    A(A),
    B(B),
}
