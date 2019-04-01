use na::Matrix4;
use na::{Isometry3, Perspective3, Point3, Vector3};

pub struct Camera {
    pub stale: bool,
    pub cache: Matrix4<f32>,
    pub pos: Point3<f32>,
    pub forward: Vector3<f32>,
    pub up: Vector3<f32>,
    pub persp: Perspective3<f32>,
}

impl Camera {
    pub fn new(pos: Point3<f32>, rot: Vector3<f32>, persp: Perspective3<f32>) -> Camera {
        Camera {
            stale: true,
            cache: Matrix4::identity(),
            pos,
            forward: Vector3::identity(),
            up: Vector3::identity(),
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
        let ymp = 0.0f32; //self.rot.y - std::f32::consts::PI / 2.0;
        Vector3::new(ymp.sin(), 0.0, ymp.cos())
    }

    pub fn resize(&mut self, size: [f32; 2]) {
        self.persp.set_aspect(size[0] / size[1]);
        self.stale = true;
    }

    pub fn mov(&mut self, x: f32, y: f32, z: f32) {
        self.pos += self.forward * z;
        self.pos += self.up * y;
        self.pos += self.right() * x;
        self.stale = true;
    }
}
