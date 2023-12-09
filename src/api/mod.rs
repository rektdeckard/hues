mod v2;

pub use v2::V2;

pub enum Version {
    V1,
    V2,
}

pub trait API {
    fn get_lights() -> Vec<crate::light::Light>;
}
