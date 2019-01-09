use super::super::*;
use super::*;
use na::{Point, Point2, Scalar, Vector, Vector2};
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Debug)]
pub struct Polygon<N: Scalar> {
    pub points: Vec<Point2<N>>,
}

pub struct EdgeIterator<'a, N: Scalar> {
    pub polygon: &'a Polygon<N>,
    pub a: usize,
}

impl<'a, N: Scalar> EdgeIterator<'a, N> {
    fn new(polygon: &'a Polygon<N>) -> Self {
        EdgeIterator { polygon, a: 0 }
    }
}

impl<'a, N: Scalar> Iterator for EdgeIterator<'a, N> {
    type Item = [Point2<N>; 2];
    fn next(&mut self) -> Option<Self::Item> {
        if self.a >= self.polygon.points.len() {
            return None;
        }

        let line = [
            self.polygon[self.a],
            self.polygon[(self.a + 1) % self.polygon.points.len()],
        ];

        self.a += 1;
        Some(line)
    }
}

impl<N: Scalar> Index<usize> for Polygon<N> {
    type Output = Point2<N>;
    fn index(&self, i: usize) -> &Point2<N> {
        &self.points[i]
    }
}

impl<N: Scalar> IndexMut<usize> for Polygon<N> {
    fn index_mut(&mut self, i: usize) -> &mut Point2<N> {
        &mut self.points[i]
    }
}

impl<N: Display + Scalar> Display for Polygon<N> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "[");
        for edge in self.edges() {
            write!(f, "[{}, {}], ", edge[0], edge[1]);
        }
        write!(f, "]");
        Ok(())
    }
}

impl<N: Scalar> Polygon<N> {
    pub fn edges<'a>(&'a self) -> EdgeIterator<'a, N> {
        EdgeIterator::new(self)
    }
}

impl Polygon<f64> {
    pub fn square(r: f64, off_x: f64, off_y: f64) -> Polygon<f64> {
        let off = Vector2::new(off_x, off_y);
        Polygon {
            points: vec![
                Point2::new(-r, -r) + off,
                Point2::new(r, -r) + off,
                Point2::new(r, r) + off,
                Point2::new(-r, r) + off,
            ],
        }
    }

    pub fn clip(&self, bound: &[Point2<f64>; 2]) -> Option<Polygon<f64>> {
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        enum Side {
            Top,
            Bottom,
            Left,
            Right,
        }
        fn clip_side(
            poly: Polygon<f64>,
            bound: &[Point2<f64>; 2],
            side: Side,
        ) -> Option<Polygon<f64>> {
            let mut res = Vec::new();
            let mut a = poly[poly.points.len() - 1];
            for b in poly.points {
                match side {
                    Side::Left => {
                        if b.x >= bound[0].x {
                            if a.x < bound[0].x {
                                let intersect = inf_line_intersection(
                                    &[a, b],
                                    &[bound[0], Point2::new(bound[0].x, bound[1].y)],
                                )
                                .unwrap();
                                match intersect {
                                    Either::A(p) => res.push(p),
                                    _ => panic!("Not Implemented: Parallel Clip"),
                                }
                            }
                            res.push(b);
                        } else if a.x >= bound[0].x {
                            let intersect = inf_line_intersection(
                                &[a, b],
                                &[bound[0], Point2::new(bound[0].x, bound[1].y)],
                            )
                            .unwrap();
                            match intersect {
                                Either::A(p) => res.push(p),
                                _ => panic!("Not Implemented: Parallel Clip"),
                            }
                        }
                    }
                    Side::Right => {
                        if b.x <= bound[1].x {
                            if a.x > bound[1].x {
                                let intersect = inf_line_intersection(
                                    &[a, b],
                                    &[Point2::new(bound[1].x, bound[0].y), bound[1]],
                                )
                                .unwrap();
                                match intersect {
                                    Either::A(p) => res.push(p),
                                    _ => panic!("Not Implemented: Parallel Clip"),
                                }
                            }
                            res.push(b);
                        } else if a.x <= bound[1].x {
                            let intersect = inf_line_intersection(
                                &[a, b],
                                &[Point2::new(bound[1].x, bound[0].y), bound[1]],
                            )
                            .unwrap();
                            match intersect {
                                Either::A(p) => res.push(p),
                                _ => panic!("Not Implemented: Parallel Clip"),
                            }
                        }
                    }
                    Side::Bottom => {
                        if b.y >= bound[0].y {
                            if a.y < bound[0].y {
                                let intersect = inf_line_intersection(
                                    &[a, b],
                                    &[bound[0], Point2::new(bound[1].x, bound[0].y)],
                                )
                                .unwrap();
                                match intersect {
                                    Either::A(p) => res.push(p),
                                    _ => panic!("Not Implemented: Parallel Clip"),
                                }
                            }
                            res.push(b);
                        } else if a.y >= bound[0].y {
                            let intersect = inf_line_intersection(
                                &[a, b],
                                &[bound[0], Point2::new(bound[1].x, bound[0].y)],
                            )
                            .unwrap();
                            match intersect {
                                Either::A(p) => res.push(p),
                                _ => panic!("Not Implemented: Parallel Clip"),
                            }
                        }
                    }
                    Side::Top => {
                        if b.y <= bound[1].y {
                            if a.y > bound[1].y {
                                let intersect = inf_line_intersection(
                                    &[a, b],
                                    &[Point2::new(bound[0].x, bound[1].y), bound[1]],
                                )
                                .unwrap();
                                match intersect {
                                    Either::A(p) => res.push(p),
                                    _ => panic!("Not Implemented: Parallel Clip"),
                                }
                            }
                            res.push(b);
                        } else if a.y <= bound[1].y {
                            let intersect = inf_line_intersection(
                                &[a, b],
                                &[Point2::new(bound[0].x, bound[1].y), bound[1]],
                            )
                            .unwrap();
                            match intersect {
                                Either::A(p) => res.push(p),
                                _ => panic!("Not Implemented: Parallel Clip"),
                            }
                        }
                    }
                }
                a = b
            }
            if res.len() == 0 {
                return None;
            }
            Some(Polygon { points: res })
        }
        clip_side(
            clip_side(
                clip_side(
                    clip_side(self.clone(), bound, Side::Left)?,
                    bound,
                    Side::Top,
                )?,
                bound,
                Side::Right,
            )?,
            bound,
            Side::Bottom,
        )
    }

    pub fn min_y(&self) -> f64 {
        let mut res = self[0][1];
        for point in &self.points {
            if res > point[1] {
                res = point[1]
            }
        }
        res
    }

    pub fn max_y(&self) -> f64 {
        let mut res = self[0][1];
        for point in &self.points {
            if res < point[1] {
                res = point[1]
            }
        }
        res
    }

    pub fn intersection_line(
        &self,
        line: [Point2<f64>; 2],
    ) -> Vec<Either<Point2<f64>, [Point2<f64>; 2]>> {
        let mut res = Vec::new();
        for edge in self.edges() {
            let intersect = line_intersection(&[edge[0], edge[1]], &line);
            if let Some(i) = intersect {
                res.push(i);
            }
        }
        res
    }

    pub fn draw(&self, buf: &mut Buffer<Color>, color: Color) {
        //println!("Drawing: {}", self);
        //println!("Clipped: {}", d);
        let min_y = self.min_y() as isize;
        let max_y = self.max_y() as isize;
        //println!("min: {}, max: {}", min_y, max_y);
        for y in min_y as isize..=max_y as isize {
            //println!("Y: {}", y);
            let scan = [
                Point2::new(0.0, y as f64),
                Point2::new(buf.size[0] as f64, y as f64),
            ];
            let line = {
                //println!("Intersecting: {}", scan);
                let intersect = self.intersection_line(scan);
                if intersect.len() != 2 {
                    continue;
                }
                //println!("Intersect: {:?}", intersect);
                [
                    match intersect[0] {
                        Either::A(p) => p,
                        _ => panic!("Intersection failure: {:?}", intersect[0]),
                    },
                    match intersect[1] {
                        Either::A(p) => p,
                        _ => panic!("Intersection failure: {:?}", intersect[1]),
                    },
                ]
            };
            let line = [
                Point2::new(line[0][0] as isize, line[0][1] as isize),
                Point2::new(line[1][0] as isize, line[1][1] as isize),
            ];
            //println!("Drawing line: {}", line);
            draw_line(buf, color, line);
            let red = [255, 125, 125, 255];
            let green = [125, 255, 125, 255];
            let blue = [125, 125, 255, 255];
            let colors = [red, green, blue];
            let mut i = 0;
            for edge in self.edges() {
                let p1 = edge[0];
                let p2 = edge[1];
                let color = colors[i % colors.len()];
                let line = [
                    Point2::new(p1[0] as isize, p1[1] as isize),
                    Point2::new(p2[0] as isize, p2[1] as isize),
                ];
                //println!("Wire {:?}: [\n{}\n{}\n]", color, line[0], line[1]);
                draw_line(buf, color, line);
                i += 1;
            }
        }
    }
}

pub fn line_rect_intersection(
    line: &[Point2<f64>; 2],
    rect: &[Point2<f64>; 2],
) -> Vec<Either<Point2<f64>, [Point2<f64>; 2]>> {
    let tl = [rect[0].x, rect[1].y].into();
    let br = [rect[1].x, rect[0].y].into();
    let edges = [[rect[0], br], [rect[0], tl], [br, rect[1]], [tl, rect[1]]];
    let mut res = Vec::new();
    for edge in edges.iter() {
        let intersect = line_intersection(&edge, &line);
        if let Some(i) = intersect {
            res.push(i);
        }
    }
    res
}
