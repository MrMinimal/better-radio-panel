pub struct FrequencyState {
    pub standby_integer_part: i16,
    pub standby_fractional_part: i16,
    pub active_integer_part: i16,
    pub active_fractional_part: i16,
}

pub struct AdfState {
    pub active_frequency: i16,
    pub standby_frequency: i16,
}

pub struct DmeState {
    pub distance: f32,
}

pub struct XpdrState {
    pub code: [i8; 4],
    pub selected_digit: usize,
}
