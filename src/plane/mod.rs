use super::*;
use na::{Point2, Scalar, Vector2};

pub mod polygon;

pub type Line2<N> = [Point2<N>; 2];

pub fn line_intersection(
    a: &[Point2<f64>; 2],
    b: &[Point2<f64>; 2],
) -> Option<Either<Point2<f64>, Line2<f64>>> {
    use super::Either::*;
    fn cross(a: &Vector2<f64>, b: &Vector2<f64>) -> f64 {
        a[0] * b[1] - a[1] * b[0]
    }
    let r = a[1] - a[0];
    let s = b[1] - b[0];
    let rxs = cross(&r, &s);
    let diff = b[0] - a[0];
    if !relative_eq!(rxs, 0.0) {
        let u = cross(&diff, &r) / rxs;
        if u >= 0.0 && u <= 1.0 {
            let t = cross(&diff, &s) / rxs;
            if t >= 0.0 && t <= 1.0 {
                let p = r * t;
                return Some(A(a[0] + p));
            }
        }
    }
    //if relative_eq!(cross(&diff, &r).abs(), 0.0) {
    //    panic!("Not Implemented: 2D Collinear Line Intersection")
    //}
    None
}

pub fn inf_line_intersection(
    a: &[Point2<f64>; 2],
    inf: &[Point2<f64>; 2],
) -> Option<Either<Point2<f64>, [Point2<f64>; 2]>> {
    use super::Either::*;
    fn cross(a: &Vector2<f64>, b: &Vector2<f64>) -> f64 {
        a[0] * b[1] - a[1] * b[0]
    }
    let r = a[1] - a[0];
    let s = inf[1] - inf[0];
    let rxs = cross(&r, &s);
    let diff = inf[0] - a[0];
    if !relative_eq!(rxs, 0.0) {
        let t = cross(&diff, &s) / rxs;
        if t >= 0.0 && t <= 1.0 {
            let p = r * t;
            return Some(A(a[0] + p));
        }
    }
    //if relative_eq!(cross(&diff, &r).abs(), 0.0) {
    //    panic!("Not Implemented: 2D Collinear Line Intersection")
    //}
    None
}

pub fn draw_rect(buf: &mut Buffer<Color>, c: Color, l: Line2<isize>) {
    let mut index = l[0][0] + l[0][1] * buf.size[0] as isize;
    let delta = l[1] - l[0];
    let dx = delta[0];
    let dy = delta[1].signum() * buf.size[0] as isize;
    //use std::time::Instant;
    //let rect_time = Instant::now();
    let mut row = Vec::new();
    row.resize_with(dx.abs() as usize + 1, || c);
    for _y in l[0][1]..=l[1][1] {
        (&mut buf.data[index as usize..=(index + dx) as usize]).copy_from_slice(&row);
        index += dy;
        // let left = index;
        // for _x in l[0][0]..=l[1][0] {
        //     buf.data[index as usize] = c;
        //     index += dx;
        // }
        // index = left + dy;
    }
    //println!("Rect Time: {:?}", rect_time.elapsed());
}

// pub fn draw_border_rect(buf: &mut Buffer<Color>, bc: Color, fc: Color, l: ILine2, t: isize) {
//     let mut index = (l[0][0] + l[0][1] * buf.width as isize) as usize;
//     for _y in l[0][1]..=l[1][1] {
//         for _x in l[0][0]..=l[1][1] {
//             buf.data[index] = fc;
//             index += 1;
//         }
//         index += buf.width;
//     }
// }

pub fn draw_line(buf: &mut Buffer<Color>, c: Color, l: Line2<isize>) {
    let delta = l[1] - l[0];
    //println!("Line [{}, {}] Delta: {}", l[0], l[1], delta);
    let (dx, dy) = (delta[0], delta[1]);
    //println!("Line [{}, {}] dx: {}, dy: {}", l[0], l[1], dx, dy);
    if dx == 0 && dy != 0 {
        draw_line_vert(buf, c, l[0], l[1][1])
    } else if dy == 0 {
        draw_line_hori(buf, c, l[0], l[1][0])
    } else if dx.abs() == dy.abs() {
        draw_line_diag(buf, c, l)
    } else {
        draw_line_brs(buf, c, dx, dy, l)
    }
}

fn draw_line_vert(buf: &mut Buffer<Color>, c: Color, mut a: Point2<isize>, mut yb: isize) {
    if a[1] > yb {
        std::mem::swap(&mut a[1], &mut yb);
    }
    let mut index = (a[0] + a[1] * buf.size[0] as isize) as usize;
    for _i in a[1]..=yb {
        buf.data[index] = c;
        index += buf.size[0];
    }
}

fn draw_line_hori(buf: &mut Buffer<Color>, c: Color, mut a: Point2<isize>, mut xb: isize) {
    if a[0] > xb {
        std::mem::swap(&mut a[0], &mut xb);
    }
    let mut index = (a[0] + a[1] * buf.size[0] as isize) as usize;
    for _i in a[0]..=xb {
        buf.data[index] = c;
        index += 1;
    }
}

fn draw_line_diag(buf: &mut Buffer<Color>, c: Color, mut l: Line2<isize>) {
    if l[0][0] > l[1][0] {
        l = [l[1], l[0]]
    }
    let mut index = l[0][0] + l[0][1] * buf.size[0] as isize;
    let dy = (l[1][1] - l[0][1]).signum() * buf.size[0] as isize;
    for _i in l[0][0]..=l[1][0] {
        buf.data[index as usize] = c;
        index += 1 + dy;
    }
}

fn draw_line_brs(buf: &mut Buffer<Color>, c: Color, dx: isize, dy: isize, l: Line2<isize>) {
    if dx.abs() > dy.abs() {
        draw_line_brs_x(buf, c, dx, dy, l)
    } else {
        draw_line_brs_y(buf, c, dx, dy, l)
    }
}

fn draw_line_brs_x(
    buf: &mut Buffer<Color>,
    c: Color,
    mut dx: isize,
    mut dy: isize,
    mut l: Line2<isize>,
) {
    if dx < 0 {
        l = [l[1], l[0]];
        dx = l[1][0] - l[0][0];
        dy = l[1][1] - l[0][1];
    }
    //println!("Drawing Line: {:?}", l);
    let sign = dy.signum();
    let mut err = (dy.abs() << 1) - dx;
    let i1 = dy.abs() << 1;
    let i2 = (dy.abs() - dx) << 1;
    let mut index = l[0][0] + l[0][1] * buf.size[0] as isize;
    for _i in l[0][0]..=l[1][0] {
        buf.data[index as usize] = c;
        index += 1;
        if err < 0 {
            err += i1;
        } else {
            index += sign * buf.size[0] as isize;
            err += i2;
        }
    }
}

fn draw_line_brs_y(
    buf: &mut Buffer<Color>,
    c: Color,
    mut dx: isize,
    mut dy: isize,
    mut l: Line2<isize>,
) {
    if dy < 0 {
        //println!("Y Initial: [{}, {}], dy: {}", l[0], l[1], dy);
        l = [l[1], l[0]];
        dy = l[1][1] - l[0][1];
        dx = l[1][0] - l[0][0];
        //println!("Y Swapped: [{}, {}], dy: {}", l[0], l[1], dy);
    }
    //println!("Drawing Line: [{}, {}], dx: {}", l[0], l[1], dx);
    let sign = dx.signum();
    let mut err = (dx.abs() << 1) - dy;
    let i1 = dx.abs() << 1;
    let i2 = (dx.abs() - dy) << 1;
    let mut index = l[0][0] + l[0][1] * buf.size[0] as isize;
    for _i in l[0][1]..=l[1][1] {
        buf.data[index as usize] = c;
        index += buf.size[0] as isize;
        if err < 0 {
            err += i1;
        } else {
            index += sign;
            err += i2;
        }
    }
}

pub fn rect_to_vertices<N: Scalar>(rect: &Line2<N>) -> [Point2<N>; 4] {
    [
        rect[0],
        [rect[1][0], rect[0][1]].into(),
        rect[1],
        [rect[0][0], rect[1][1]].into(),
    ]
}

pub fn rect_to_edges<N: Scalar>(rect: &Line2<N>) -> [Line2<N>; 4] {
    let verts = rect_to_vertices(rect);
    [
        [verts[0], verts[1]],
        [verts[1], verts[2]],
        [verts[2], verts[3]],
        [verts[3], verts[0]],
    ]
}

pub fn aabb_collision(r1: &Line2<f64>, r2: &Line2<f64>) -> Vec<Either<Point2<f64>, Line2<f64>>> {
    let e1 = rect_to_edges(r1);
    let e2 = rect_to_edges(r2);
    let mut res = Vec::new();
    for edge in e1.iter() {
        for redge in e2.iter() {
            if let Some(e) = line_intersection(&edge, &redge) {
                res.push(e)
            }
        }
    }
    res
}

pub fn lb_clip(
    line: &[Point2<f64>; 2],
    bound: &[Point2<f64>; 2],
) -> Option<([Point2<f64>; 2], [bool; 2])> {
    fn clip(p: f64, q: f64, mut t: [f64; 2]) -> Option<[f64; 2]> {
        let r = q / p;
        if relative_eq!(p, 0.0) && q < 0.0 {
            return None;
        }
        if p < 0.0 {
            if r > t[1] {
                return None;
            }
            if r > t[0] {
                t[0] = r
            }
        } else {
            if r < t[0] {
                return None;
            }
            if r < t[1] {
                t[1] = r
            }
        }
        Some(t)
    }
    let mut bound = bound.clone();
    // make sure that the line describing the bound is directed up and right
    if bound[0][0] > bound[1][0] {
        let temp = bound[1][0];
        bound[1][0] = bound[0][0];
        bound[0][0] = temp;
    }
    if bound[0][1] > bound[1][1] {
        let temp = bound[1][1];
        bound[1][1] = bound[0][1];
        bound[0][1] = temp;
    }
    let delta = line[1] - line[0];
    let mut tmin = 0.0;
    let mut tmax = 1.0;

    // left edge (doing some fancy footwork here to avoid doing math because we know tmin & tmax at this point)
    {
        let p = -delta.x;
        let q = -(bound[0].x - line[0].x);
        //let r = q / p;
        if relative_eq!(p, 0.0) && q < 0.0 {
            return None;
        }
        if p < 0.0 {
            // if r > 1
            if q < p {
                return None;
            }
            // if 0 < r < 1
            if q < 0.0 {
                tmin = q / p
            }
        } else {
            // if r < 0
            if q < 0.0 {
                return None;
            }
            // if 0 < r < 1
            if q < p {
                tmax = q / p;
            }
        }
    }
    let clip = clip(
        delta.y,
        bound[1].y - line[0].y,
        clip(
            -delta.y,
            bound[0].y - line[0].y,
            clip(delta.x, bound[1].x - line[0].x, [tmin, tmax])?,
        )?,
    )?;

    let mut clipped = [false, false];

    let mut res = line.clone();

    if relative_eq!(clip[0], 0.0) {
        clipped[0] = true;
        res[0].x = line[0].x + clip[0] * delta.x;
        res[0].y = line[0].y + clip[0] * delta.y;
    }
    if relative_eq!(clip[1], 0.0) {
        clipped[1] = true;
        res[1].x = line[0].x + clip[1] * delta.x;
        res[1].y = line[0].y + clip[1] * delta.y;
    }
    Some((res, clipped))
}
