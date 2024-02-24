/// Make sure values stay within min and max
/// Wraps around on both ends
pub fn wrap<T: std::cmp::PartialOrd + std::ops::Sub<Output = T> + std::ops::Add<Output = T>>(
    value: T,
    min: T,
    max: T,
) -> T {
    if value < min {
        max - (min - value)
    } else if value >= max {
        min + (value - max)
    } else {
        value
    }
}

#[cfg(test)]
mod wrap_tests {
    use super::*;

    #[test]
    fn test_in_range() {
        assert_eq!(wrap(120, 110, 140), 120);
    }

    #[test]
    fn test_on_min() {
        assert_eq!(wrap(110, 110, 140), 110);
    }

    #[test]
    fn test_on_max() {
        assert_eq!(wrap(140, 110, 140), 110);
    }

    #[test]
    fn test_above_max() {
        assert_eq!(wrap(150, 110, 140), 120);
    }

    #[test]
    fn test_below_min() {
        assert_eq!(wrap(90, 110, 140), 120);
    }
}