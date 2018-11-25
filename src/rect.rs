use std::sync::Arc;

use crate::*;

macro_rules! vector3_xyz {
    ($x:expr, $y:expr, $z:expr) => {
        Vector3::new($x, $y, $z)
    };
}

macro_rules! vector3_zxy {
    ($z:expr, $x:expr, $y:expr) => {
        Vector3::new($x, $y, $z)
    };
}

macro_rules! vector3_yzx {
    ($y:expr, $z:expr, $x:expr) => {
        Vector3::new($x, $y, $z)
    };
}

macro_rules! rect {
    ($xy_rect:ident, $x:ident, $y:ident, $v3:ident) => {
        pub struct $xy_rect {
            material: Arc<Material>,
            $x: (f32, f32),
            $y: (f32, f32),
            k: f32,
        }

        impl $xy_rect {
            pub fn new<T: Into<Arc<Material>>>(
                $x: (f32, f32),
                $y: (f32, f32),
                k: f32,
                material: T,
            ) -> $xy_rect {
                $xy_rect {
                    material: material.into(),
                    $x,
                    $y,
                    k,
                }
            }
        }

        unsafe impl Send for $xy_rect {}

        unsafe impl Sync for $xy_rect {}

        impl Hittable for $xy_rect {
            fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
                let t = (self.k - ray.origin().z()) / ray.direction().z();
                if t < t_min || t > t_max {
                    return None;
                }

                let $x = ray.origin().$x() + ray.direction().$x() * t;
                if $x < self.$x.0 || $x > self.$x.1 {
                    return None;
                }

                let $y = ray.origin().$y() + ray.direction().$y() * t;
                if $y < self.$y.0 || $y > self.$y.1 {
                    return None;
                }

                Some(HitRecord {
                    t,
                    position: $v3!($x, $y, self.k),
                    normal: $v3!(0.0, 0.0, 1.0),
                    material: self.material.clone(),
                    u: ($x - self.$x.0) / (self.$x.1 - self.$x.0),
                    v: ($y - self.$y.0) / (self.$y.1 - self.$y.0),
                })
            }

            fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
                Some(AABB::new(
                    $v3!(self.$x.0, self.$y.0, self.k - 0.00001),
                    $v3!(self.$x.1, self.$y.1, self.k + 0.00001),
                ))
            }
        }
    };
}

rect!(XyRect, x, y, vector3_xyz);
rect!(YzRect, y, z, vector3_yzx);
rect!(ZxRect, z, x, vector3_zxy);
