#[derive(Copy, Clone, Debug)]
pub struct Frequency {
    pub integer: i16,
    pub fraction: i16,
}

// Formats a given frequency to the 00.000 or 000.00 format
pub fn format_frequency(freq: Frequency, fractional_digits: u8) -> String {
    match fractional_digits {
        2 => {
            return format!(
                "{integer}.{fraction}",
                integer = freq.integer,
                fraction = format!("{:03}", freq.fraction).chars().take(2).collect::<String>()
            );
        }
        3 => {
            return format!(
                "{integer}.{fraction}",
                integer = freq.integer.to_string()[1..].to_string(),
                fraction = format!("{:03}", freq.fraction),
            );
        }
        _ => {
            panic!("Can't format frequencies other than 2 or 3 digits")
        }
    }
}

#[cfg(test)]
mod frequency_formatting_tests {
    use crate::{format_frequency, Frequency};

    #[test]
    fn test_two_digits() {
        let freq1: Frequency = Frequency{integer: 108, fraction: 0};
        let freq2: Frequency = Frequency{integer: 108, fraction: 50};
        let freq3: Frequency = Frequency{integer: 108, fraction: 100};
        let freq4: Frequency = Frequency{integer: 108, fraction: 150};
        let freq5: Frequency = Frequency{integer: 108, fraction: 200};
        assert_eq!(format_frequency(freq1, 2), "108.00");
        assert_eq!(format_frequency(freq2, 2), "108.05");
        assert_eq!(format_frequency(freq3, 2), "108.10");
        assert_eq!(format_frequency(freq4, 2), "108.15");
        assert_eq!(format_frequency(freq5, 2), "108.20");
    }

    #[test]
    fn test_three_digits() {
        let freq1: Frequency = Frequency{integer: 108, fraction: 0};
        let freq2: Frequency = Frequency{integer: 108, fraction: 50};
        let freq3: Frequency = Frequency{integer: 108, fraction: 100};
        let freq4: Frequency = Frequency{integer: 108, fraction: 150};
        let freq5: Frequency = Frequency{integer: 108, fraction: 200};
        assert_eq!(format_frequency(freq1, 3), "08.000");
        assert_eq!(format_frequency(freq2, 3), "08.050");
        assert_eq!(format_frequency(freq3, 3), "08.100");
        assert_eq!(format_frequency(freq4, 3), "08.150");
        assert_eq!(format_frequency(freq5, 3), "08.200");
    }
}
