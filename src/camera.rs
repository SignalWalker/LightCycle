use volume::*;

use na::{normalize, Isometry3, Perspective3, Point3, Translation3, UnitQuaternion, Vector3};

pub struct Camera {
    pub free: bool,
    pub pos: Point3<f64>,
    pub rot: Vector3<f64>,
    pub forward: Vector3<f64>,
    pub up: Vector3<f64>,
    pub right: Vector3<f64>,
    pub persp: Perspective3<f64>,
}

impl Camera {
    pub fn new(pos: Point3<f64>, rot: Vector3<f64>, persp: Perspective3<f64>) -> Camera {
        let mut res = Camera {
            free: false,
            pos,
            rot,
            persp,
            forward: Vector3::identity(),
            up: Vector3::identity(),
            right: Vector3::identity(),
        };
        res.update();
        res
    }

    pub fn update(&mut self) {
        let cosx = self.rot.x.cos();
        self.forward = Vector3::new(
            cosx * self.rot.y.sin(),
            self.rot.x.sin(),
            cosx * self.rot.y.cos(),
        );
        let ymp = self.rot.y - std::f64::consts::PI / 2.0;
        self.right = Vector3::new(ymp.sin(), 0.0, ymp.cos());
        self.up = self.right.cross(&self.forward);
    }

    pub fn iso(&self) -> Isometry3<f64> {
        if self.free {
            Isometry3::look_at_lh(&self.pos, &(self.pos + self.forward), &self.up)
        } else {
            Isometry3::look_at_lh(&self.pos, &Point3::new(0.0, 0.0, 0.0), &Vector3::y())
        }
    }

    pub fn look_at(&mut self, p: Point3<f64>) {}

    pub fn mov(&mut self, vec: &Vector3<f64>) {
        self.pos += self.forward * vec[2];
        self.pos += self.up * vec[1];
        self.pos += self.right * vec[0];
        self.update()
    }

    pub fn rot(&mut self, x: f64, y: f64, z: f64) {
        self.rot.x = //x;
        (-std::f64::consts::PI / 2.0).max((std::f64::consts::PI / 2.0).min(self.rot.x + x));
        self.rot.y += y;
        //(-std::f64::consts::PI / 2.0).max((std::f64::consts::PI / 2.0).min(self.rot.y + y));
        self.rot.z += z;
        //(-std::f64::consts::PI / 2.0).max((std::f64::consts::PI / 2.0).min(self.rot.z + z));
        self.update()
    }
}
