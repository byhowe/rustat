#![feature(adt_const_params)]

pub mod erf;
mod integration;

pub use integration::midpoint;
pub use integration::midpoint_w;
pub use integration::simpsons;
pub use integration::simpsons_w;
pub use integration::trapezoid;
pub use integration::trapezoid_w;
