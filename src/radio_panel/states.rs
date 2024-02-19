use crate::radio_panel::frequency::*;

pub struct FrequencyState {
    pub standby_freq: Frequency,
    pub active_freq: Frequency,
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
