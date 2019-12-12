use na::Matrix3;

use std::iter::FromIterator;
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug)]
pub struct Polyhedron<P> {
    pub points: Vec<P>,
    pub faces: Vec<[u16; 3]>,
}

impl<P> Polyhedron<P> {
    pub fn map<O>(self, f: fn(P) -> O) -> Polyhedron<O> {
        Polyhedron {
            points: self.points.into_iter().map(f).collect::<Vec<_>>(),
            faces: self.faces,
        }
    }

    pub fn from<O: Into<P>>(o: Polyhedron<O>) -> Self {
        o.map(Into::into)
    }

    pub fn into<O>(self) -> Polyhedron<O>
    where
        P: Into<O>,
    {
        self.map(Into::into)
    }

    pub fn points(&self) -> std::slice::Iter<'_, P> {
        self.points.iter()
    }

    pub fn points_mut(&mut self) -> std::slice::IterMut<'_, P> {
        self.points.iter_mut()
    }

    pub fn faces(&self) -> std::slice::Iter<'_, [u16; 3]> {
        self.faces.iter()
        // .map(|f| {
        //     [
        //         self.points[f[0] as usize],
        //         self.points[f[1] as usize],
        //         self.points[f[2] as usize],
        //     ]
        // })
    }

    pub fn faces_mut(&mut self) -> std::slice::IterMut<'_, [u16; 3]> {
        self.faces.iter_mut()
    }
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

impl<P: PartialEq + Clone> FromIterator<[P; 3]> for Polyhedron<P> {
    fn from_iter<I: IntoIterator<Item = [P; 3]>>(iter: I) -> Self {
        let mut points = Vec::new();
        let faces =
            iter.into_iter()
                .map(|tri| {
                    let mut map = tri.iter().map(|p| {
                        match points.iter().enumerate().find(|(_, o)| *p == **o) {
                            Some((i, _)) => i,
                            None => {
                                points.push(p.clone());
                                points.len() - 1
                            }
                        }
                    });
                    [
                        map.next().unwrap() as u16,
                        map.next().unwrap() as u16,
                        map.next().unwrap() as u16,
                    ]
                })
                .collect::<Vec<_>>();
        Self { points, faces }
    }
}

impl Polyhedron<[f32; 3]> {
    pub fn cube() -> Self {
        Polyhedron {
            points: vec![
                // back
                [0.5, -0.5, -0.5],
                [-0.5, -0.5, -0.5],
                [-0.5, 0.5, -0.5],
                [0.5, 0.5, -0.5],
                // front
                [-0.5, -0.5, 0.5],
                [0.5, -0.5, 0.5],
                [0.5, 0.5, 0.5],
                [-0.5, 0.5, 0.5],
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
                [-0.5, 0.0, -0.5],
                [0.5, 0.0, -0.5],
                [0.5, 0.0, 0.5],
                [-0.5, 0.0, 0.5],
                // top
                [0.0, 0.5, 0.0],
                // bottom
                [0.0, -0.5, 0.0],
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
            points: vec![[0.5, -0.5, -0.5], [-0.5, -0.5, -0.5], [0.0, -0.5, 0.5]],
            faces: vec![[0, 1, 2]],
        }
    }

    pub fn quad() -> Self {
        Polyhedron {
            points: vec![
                [0.5, -0.5, -0.5],
                [-0.5, -0.5, -0.5],
                [-0.5, -0.5, 0.5],
                [0.5, -0.5, 0.5],
            ],
            faces: vec![[0, 1, 2], [2, 3, 0]],
        }
    }

    pub fn ramp_tri() -> Self {
        Polyhedron {
            points: vec![[0.5, -0.5, -0.5], [-0.5, -0.5, -0.5], [-0.5, 0.5, 0.5]],
            faces: vec![[0, 1, 2]],
        }
    }

    pub fn ramp_quad() -> Self {
        Polyhedron {
            points: vec![
                [0.5, -0.5, -0.5],
                [-0.5, -0.5, -0.5],
                [-0.5, 0.5, 0.5],
                [0.5, 0.5, 0.5],
            ],
            faces: vec![[0, 1, 2], [2, 3, 0]],
        }
    }

    pub fn inertia_moment(&self) -> Matrix3<f32> {
        unimplemented!()
    }
}
