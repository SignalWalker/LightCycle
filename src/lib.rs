#[macro_use]
pub extern crate approx;
pub extern crate nalgebra as na;

use na::Point2;
use std::ops::{Index, IndexMut};

pub mod camera;
pub mod plane;
pub mod volume;

use plane::*;
use volume::*;

pub type Color = [u8; 4];

#[derive(Copy, Clone, Debug)]
pub enum Either<A, B> {
    A(A),
    B(B),
}

pub struct Buffer<'a, T: 'a> {
    pub data: &'a mut [T],
    pub size: Point2<usize>,
}

impl<'a, T> Buffer<'a, T> {
    pub fn from_slice(size: Point2<usize>, data: &'a mut [T]) -> Buffer<T> {
        // if you put in the wrong width & height, you are a fool
        // and i will not save you
        Buffer { data, size }
    }
}

impl<'a, T> Index<[usize; 2]> for Buffer<'a, T> {
    type Output = T;

    fn index(&self, i: [usize; 2]) -> &T {
        &self.data[i[0] + i[1] * self.size[0]]
    }
}

impl<'a, T> IndexMut<[usize; 2]> for Buffer<'a, T> {
    fn index_mut(&mut self, i: [usize; 2]) -> &mut T {
        &mut self.data[i[0] + i[1] * self.size[0]]
    }
}

pub fn break_color_slice(vec: &[Color]) -> &[u8] {
    unsafe {
        let ptr: *const u8 = vec.as_ptr() as *const u8;
        std::slice::from_raw_parts(ptr, vec.len() * 4)
    }
}

pub unsafe fn break_color_slice_mut(vec: &mut [Color]) -> &mut [u8] {
    let ptr: *mut u8 = vec.as_ptr() as *mut u8;
    std::slice::from_raw_parts_mut(ptr, vec.len() * 4)
}

pub fn clear(buf: &mut Buffer<Color>, color: Color) {
    //use std::time::Instant;
    //let clear_time = Instant::now();

    if color[3] == 0 {
        unsafe {
            std::ptr::write_bytes(buf.data.as_mut_ptr(), 0, buf.data.len());
        }
    } else {
        let mut index = 0;
        let mut row = Vec::new();
        row.resize_with(buf.size[0], || color);
        for _y in 0..buf.size[1] {
            (&mut buf.data[index..(index + buf.size[0])]).copy_from_slice(&row);
            index += buf.size[0];
        }
    }
    //println!("Clear Time: {:?}", clear_time.elapsed());
}
