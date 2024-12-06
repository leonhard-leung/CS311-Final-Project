/// Adds two integers.
///
/// # Arguments
/// * `x` - The first integer.
/// * `y` - The second integer.
///
/// # Example
/// ```
/// use _9365_Team02_Rust_ProjectArtifacts::add;
/// let result = add(2,3);
/// assert_eq!(result, 5);
/// ```
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}


/// Subtracts one integer from another.
///
/// # Arguments
/// * `x` - The integer to subtract from.
/// * `y` - The integer to subtract.
///
/// # Example
/// ```
/// use _9365_Team02_Rust_ProjectArtifacts::subtract;
/// let result = subtract(5, 2);
/// assert_eq!(result, 3);
/// ```
pub fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

/// Multiplies two integers.
///
/// # Arguments
/// * `x` - The first integer.
/// * `y` - The second integer.
///
/// # Example
/// ```
/// use _9365_Team02_Rust_ProjectArtifacts::multiply;
/// let result = multiply(4, 3);
/// assert_eq!(result, 12);
/// ```
pub fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

/// Divides one integer by another.
///
/// Returns `None` if division by zero is attempted.
///
/// # Arguments
/// * `x` - The numerator.
/// * `y` - The denominator.
///
/// # Example
/// ```
/// use _9365_Team02_Rust_ProjectArtifacts::divide;
/// let result = divide(10, 2);
/// assert_eq!(result, Some(5));
///
/// let result = divide(10, 0);
/// assert_eq!(result, None);
/// ```
pub fn divide(x: i32, y: i32) -> Option<i32> {
    if y == 0
        {
            None
        }
    else
        {
            Some(x / y)
        }
}