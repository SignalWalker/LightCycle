use na::{Matrix3, Point3};
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug)]
pub struct Polyhedron<P> {
    pub points: Vec<P>,
    pub faces: Vec<[u16; 3]>,
}

impl<P> Index<usize> for Polyhedron<P> {
    type Output = P;
    fn index(&self, i: usize) -> &P {
        &self.points[i]
    }
}

impl<P> IndexMut<usize> for Polyhedron<P> {
    fn index_mut(&mut self, i: usize) -> &mut P {
        &mut self.points[i]
    }
}

impl Polyhedron<Point3<f32>> {
    pub fn cube() -> Self {
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

    pub fn octohedron() -> Self {
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

    pub fn triangle() -> Self {
        Polyhedron {
            points: vec![
                [1.0, -1.0, 0.0].into(),
                [0.0, 1.0, 0.0].into(),
                [-1.0, -1.0, 0.0].into(),
            ],
            faces: vec![[0, 1, 2]],
        }
    }

    pub fn plane() -> Self {
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
