/// Returns the square root of a number rounded to the nearest integer.
///
/// # Arguments
///
/// * `num` - The number to find the square root of
///
/// # Returns
///
/// The square root of the number rounded to the nearest integer
///
pub fn roundedSqrt(&num: f64) -> i32 {
    return num.sqrt().round() as i32;
}
