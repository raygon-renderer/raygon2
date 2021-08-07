pub mod matrix4;
pub mod point3;
pub mod quaternion;
pub mod ray3;
pub mod tangent;
pub mod transform3;
pub mod vector3;

pub use self::{matrix4::Matrix4, point3::Point3, quaternion::Quaternion, ray3::Ray3, transform3::Transform3, vector3::Vector3};

pub type Error3<S> = Vector3<S>;
