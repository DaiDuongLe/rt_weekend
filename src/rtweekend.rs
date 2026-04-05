use core::f64;

// Constants
pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = f64::consts::PI;

// Utility functions
pub fn degrees_to_radians(degrees: f64) {
    degrees * PI / 180.0;
}
