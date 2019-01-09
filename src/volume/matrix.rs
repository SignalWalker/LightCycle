use super::super::plane::{matrix::*, *};
use super::super::rotation::*;
use super::{polyhedron::*, *};
use std::convert::From;
use std::fmt::Debug;
use std::ops::{Index, Mul, MulAssign};

pub type DPoint3 = Point3<f64>;

#[derive(Copy, Clone, Debug)]
pub struct Matrix4([f64; 16]);

impl Display for Matrix4 {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "[\n{} {} {} {}\n{} {} {} {}\n{} {} {} {}\n{} {} {} {}\n]",
            self[0],
            self[1],
            self[2],
            self[3],
            self[4],
            self[5],
            self[6],
            self[7],
            self[8],
            self[9],
            self[10],
            self[11],
            self[12],
            self[13],
            self[14],
            self[15]
        )
    }
}

impl Matrix4 {
    pub fn identity() -> Matrix4 {
        Matrix4([
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn scale(s: Point4<f64>) -> Matrix4 {
        Matrix4([
            s[0], 0.0, 0.0, 0.0, 0.0, s[1], 0.0, 0.0, 0.0, 0.0, s[2], 0.0, 0.0, 0.0, 0.0, s[3],
        ])
    }

    pub fn translation(p: Point3<f64>) -> Matrix4 {
        Matrix4([
            1.0, 0.0, 0.0, p[0], 0.0, 1.0, 0.0, p[1], 0.0, 0.0, 1.0, p[2], 0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn from_quaternion(q: Point4<f64>) -> Matrix4 {
        let x2 = q[0].powi(2);
        let y2 = q[1].powi(2);
        let z2 = q[2].powi(2);
        let xy = q[0] * q[1];
        let xz = q[0] * q[2];
        let xw = q[0] * q[3];
        let yz = q[1] * q[2];
        let yw = q[1] * q[3];
        let zw = q[2] * q[3];
        Matrix4([
            1.0 - 2.0 * (y2 - z2),
            2.0 * (xy + zw),
            2.0 * (xz - yw),
            0.0,
            2.0 * (xy - zw),
            1.0 - 2.0 * (x2 - z2),
            2.0 * (yz + xw),
            0.0,
            2.0 * (xz + yw),
            2.0 * (yz - xw),
            1.0 - 2.0 * (x2 - y2),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ])
        //println!("Quaternion {} to matrix: {}", q, res);
        //res
    }

    pub fn perspective(width: f64, height: f64, near: f64, far: f64) -> Matrix4 {
        let nfmn = -(far - near);
        Matrix4([
            near / width,
            0.0,
            0.0,
            0.0,
            0.0,
            near / height,
            0.0,
            0.0,
            0.0,
            0.0,
            (far + near) / nfmn,
            (2.0 * far * near) / nfmn,
            0.0,
            0.0,
            1.0,
            0.0,
        ])
    }

    pub fn orthographic(width: f64, height: f64, near: f64, far: f64) -> Matrix4 {
        let fmn = far - near;
        Matrix4([
            1.0 / width,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0 / height,
            0.0,
            0.0,
            0.0,
            0.0,
            2.0 / fmn,
            (far + near) / fmn,
            0.0,
            0.0,
            0.0,
            1.0,
        ])
    }
}

impl Index<usize> for Matrix4 {
    type Output = f64;
    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl IndexMut<usize> for Matrix4 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
    }
}

impl Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;
    fn mul(self, r: Matrix4) -> Matrix4 {
        Matrix4([
            self[0] * r[0] + self[1] * r[4] + self[2] * r[8] + self[3] * r[12],
            self[0] * r[1] + self[1] * r[5] + self[2] * r[9] + self[3] * r[13],
            self[0] * r[2] + self[1] * r[6] + self[2] * r[10] + self[3] * r[14],
            self[0] * r[3] + self[1] * r[7] + self[2] * r[11] + self[3] * r[15],
            self[4] * r[0] + self[5] * r[4] + self[6] * r[8] + self[7] * r[12],
            self[4] * r[1] + self[5] * r[5] + self[6] * r[9] + self[7] * r[13],
            self[4] * r[2] + self[5] * r[6] + self[6] * r[10] + self[7] * r[14],
            self[4] * r[3] + self[5] * r[7] + self[6] * r[11] + self[7] * r[15],
            self[8] * r[0] + self[9] * r[4] + self[10] * r[8] + self[11] * r[12],
            self[8] * r[1] + self[9] * r[5] + self[10] * r[9] + self[11] * r[13],
            self[8] * r[2] + self[9] * r[6] + self[10] * r[10] + self[11] * r[14],
            self[8] * r[3] + self[9] * r[7] + self[10] * r[11] + self[11] * r[15],
            self[12] * r[0] + self[13] * r[4] + self[14] * r[8] + self[15] * r[12],
            self[12] * r[1] + self[13] * r[5] + self[14] * r[9] + self[15] * r[13],
            self[12] * r[2] + self[13] * r[6] + self[14] * r[10] + self[15] * r[14],
            self[12] * r[3] + self[13] * r[7] + self[14] * r[11] + self[15] * r[15],
        ])
    }
}

impl Mul<DPoint3> for Matrix3 {
    type Output = DPoint3;
    fn mul(self, r: DPoint3) -> DPoint3 {
        Point3([
            self[0] * r.0[0] + self[1] * r.0[1] + self[2] * r.0[2],
            self[3] * r.0[0] + self[4] * r.0[1] + self[5] * r.0[2],
            self[6] * r.0[0] + self[7] * r.0[1] + self[8] * r.0[2],
        ])
    }
}

impl Mul<DPoint2> for Matrix3 {
    type Output = DPoint3;
    fn mul(self, r: DPoint2) -> DPoint3 {
        self * Point3([r[0], r[1], 1.0])
    }
}

impl Mul<Point4<f64>> for Matrix4 {
    type Output = Point4<f64>;
    fn mul(self, r: Point4<f64>) -> Self::Output {
        Point4([
            self[0] * r.0[0] + self[1] * r.0[1] + self[2] * r.0[2] + self[3] * r.0[3],
            self[4] * r.0[0] + self[5] * r.0[1] + self[6] * r.0[2] + self[7] * r.0[3],
            self[8] * r.0[0] + self[9] * r.0[1] + self[10] * r.0[2] + self[11] * r.0[3],
            self[12] * r.0[0] + self[13] * r.0[1] + self[14] * r.0[2] + self[15] * r.0[3],
        ])
    }
}

impl Mul<Point3<f64>> for Matrix4 {
    type Output = Point4<f64>;
    fn mul(self, r: Point3<f64>) -> Self::Output {
        self * Point4([r.0[0], r.0[1], r.0[2], 1.0])
    }
}

impl Mul<Polyhedron<f64>> for Matrix4 {
    type Output = Polyhedron<f64>;
    fn mul(self, mut r: Polyhedron<f64>) -> Self::Output {
        for point in &mut r.points {
            *point = (self * *point).xyz();
        }
        r
    }
}

mod test {
    use super::*;

    #[test]
    fn translate() {
        let p = Point3([0.0, 0.0, 0.0]);
        let mat = Matrix4::translation(Point3([1.0, 2.0, 3.0]));
        let res = mat * p;
        //println!("Res4: {}", res);
        let res = res.xyz() / res[3];
        println!("Translate: {}", res);
        assert_eq!(res, Point3([1.0, 2.0, 3.0]))
    }

    #[test]
    fn scale() {
        let p = Point3([1.0, 2.0, 3.0]);
        let mat = Matrix4::scale(Point4([2.0, 3.0, 4.0, 1.0]));
        let res = mat * p;
        //println!("Res4: {}", res);
        let res = res.xyz() / res[3];
        println!("Scale: {}", res);
        assert_eq!(res, Point3([2.0, 6.0, 12.0]))
    }

    #[test]
    fn translate_scale() {
        let res = Matrix4::translation(Point3([1.0, 2.0, 3.0]))
            * Matrix4::scale(Point4([2.0, 3.0, 4.0, 1.0]))
            * Point3([1.0, 2.0, 3.0]);
        //println!("Res4: {}", res);
        let res = res.xyz() / res[3];
        println!("Scale & Translate: {}", res);
        assert_eq!(res, Point3([3.0, 8.0, 15.0]))
    }

    #[test]
    fn rotate() {
        let q = Point4::axis_angle(Point3([1.0, 0.0, 0.0]), 2.0);
        println!("Quaternion: {}, mag: {}", q, q.mag());
        let mat = Matrix4::from_quaternion(q);
        println!("QMat: {}", mat);
        let res = mat * Point3([1.0, 1.0, 1.0]);
        //println!("Res4: {}", res);
        let res = res.xyz() / res[3];
        println!("Rotate: {}", res);
        assert_eq!(res, Point3([3.0, 8.0, 15.0]))
    }

}
