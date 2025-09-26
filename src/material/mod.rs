pub mod dielectric;
pub mod lambertian;
pub mod metal;
pub mod rainbow;
pub mod traits;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;
pub use traits::Material;
pub use rainbow::Rainbow;
