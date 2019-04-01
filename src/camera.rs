use na::Matrix4;
use na::{Isometry3, Perspective3, Point3, UnitQuaternion, Vector3};

#[derive(Debug)]
pub struct Camera {
    pub stale: bool,
    pub cache: Matrix4<f32>,
    pub pos: Point3<f32>,
    pub forward: Vector3<f32>,
    pub up: Vector3<f32>,
    pub persp: Perspective3<f32>,
}

impl Camera {
    pub fn new(pos: Point3<f32>, persp: Perspective3<f32>) -> Camera {
        Camera {
            stale: true,
            cache: Matrix4::identity(),
            pos,
            forward: Vector3::new(0.0, 0.0, 1.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            persp,
        }
    }

    pub fn fresh_mat(&mut self) -> &Matrix4<f32> {
        if self.stale {
            self.cache = self.persp.as_matrix()
                * Isometry3::face_towards(&self.pos, &(self.pos + self.forward), &self.up)
                    .to_homogeneous();
            self.stale = false;
        }
        &self.cache
    }

    pub fn right(&self) -> Vector3<f32> {
        self.up.cross(&self.forward) // (1, 0, 0) if up is (0, 1, 0) and forward is (0, 0, 1)
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.persp.set_aspect(width / height);
        self.stale = true;
    }

    pub fn mov(&mut self, x: f32, y: f32, z: f32) {
        self.pos += self.forward * z;
        self.pos += self.up * y;
        self.pos += self.right() * x;
        self.stale = true;
    }

    pub fn rot(&mut self, scaled_axis: &Vector3<f32>) {
        let q = UnitQuaternion::from_scaled_axis(*scaled_axis);
        self.forward = q * self.forward.normalize();
        self.up = q * self.up.normalize();
        self.stale = true;
    }

    pub fn look_at(&mut self, pos: &Point3<f32>) {
        let old = self.forward;
        self.forward = (pos - self.pos).normalize();
        self.up =
            (UnitQuaternion::rotation_between(&old, &self.forward).unwrap() * self.up).normalize();
        self.stale = true;
    }
}
