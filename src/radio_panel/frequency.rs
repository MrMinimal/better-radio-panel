#[derive(Copy, Clone, Debug)]
pub struct Frequency {
    pub integer: i16,
    pub fraction: i16,
}

pub fn format_frequency(freq: Frequency, fractional_digits: u8) -> String {
    match fractional_digits {
        2 => {
            return format!(
                "{integer}.{fraction}",
                integer = freq.integer,
                fraction = format!("{:03}", 950).chars().take(2).collect::<String>()
            );
        }
        3 => {
            return format!(
                "{integer}.{fraction}",
                integer = freq.integer.to_string()[1..].to_string(),
                fraction = format!("{:03}", 0),
            );
        }
        _ => {
            panic!("sorryyyy")
        }
    }
}
