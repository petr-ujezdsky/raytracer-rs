use crate::vec3::Vec3;

/// Orthonormal basis
#[derive(Debug, Copy, Clone, Default)]
pub struct Onb {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl Onb {
    pub fn new(n: Vec3) -> Onb {
        let w = n.unit_vector();
        let a = if w.x.abs() > 0.9 { Vec3::new(0.0, 1.0, 0.0) } else { Vec3::new(1.0, 0.0, 0.0) };
        let v = w.cross(a).unit_vector();
        let u = w.cross(v);

        Onb { u, v, w }
    }

    pub fn transform(&self, v: Vec3) -> Vec3 {
        self.u * v.x + self.v * v.y + self.w * v.z
    }

    // pub fn new(u: Vec3, v: Vec3, w: Vec3) -> Onb {
    //     Onb { u, v, w }
    // }
}
