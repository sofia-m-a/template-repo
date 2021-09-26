/// Returns true!
///
/// A useless function used for testing that CI works.
///
/// # Examples
/// ```
/// # use template_lib::returns_true;
/// assert!(returns_true());
/// ```
pub fn returns_true() -> bool {
    true
}

pub fn hello_world() {
    println!("Hello, World!");
}

#[cfg(test)]
mod tests {
    use super::returns_true;

    #[test]
    fn template_unit_test() {
        assert!(returns_true());
    }
}
