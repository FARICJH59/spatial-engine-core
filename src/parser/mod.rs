// Authoritative Scene Graph Parser Core
pub fn validate_scale(x: f32, y: f32, z: f32) -> Result<(), &'static str> {
    if x <= 0.0 || y <= 0.0 || z <= 0.0 {
        return Err("Invalid spatial scale dimensions.");
    }
    Ok(())
}
