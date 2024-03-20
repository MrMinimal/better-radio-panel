use crate::radio_panel::frequency::*;

pub struct InstrumentStates {
    pub com1_state: FrequencyState,
    pub com2_state: FrequencyState,
    pub nav1_state: FrequencyState,
    pub nav2_state: FrequencyState,
    pub adf_state: AdfState,
    pub dme_state: DmeState,
    pub xpdr_state: XpdrState,
    pub autopilot_state: AutopilotState,
}

pub struct FrequencyState {
    pub standby_freq: Frequency,
    pub active_freq: Frequency,
}

pub struct AdfState {
    pub active_frequency: i16,
    pub standby_frequency: i16,
}

pub struct DmeState {
    pub distance: f64,
}

pub struct XpdrState {
    pub code: [i8; 4],
    pub selected_digit: usize,
}

pub struct AutopilotState {
    pub airspeed: i16,
    pub heading: i16,
    pub altitude: i32,
    pub vertical_speed: i16,
    pub selected_setting: AutopilotValue,
}

pub enum AutopilotValue {
    Altitude,
    VerticalSpeed,
}

pub fn instruments_default_state() -> InstrumentStates {
    InstrumentStates {
        com1_state: FrequencyState {
            standby_freq: Frequency {
                integer: 118,
                fraction: 000,
            },
            active_freq: Frequency {
                integer: 118,
                fraction: 000,
            },
        },
        com2_state: FrequencyState {
            standby_freq: Frequency {
                integer: 118,
                fraction: 000,
            },
            active_freq: Frequency {
                integer: 118,
                fraction: 000,
            },
        },
        nav1_state: FrequencyState {
            standby_freq: Frequency {
                integer: 108,
                fraction: 000,
            },
            active_freq: Frequency {
                integer: 108,
                fraction: 000,
            },
        },
        nav2_state: FrequencyState {
            standby_freq: Frequency {
                integer: 108,
                fraction: 000,
            },
            active_freq: Frequency {
                integer: 108,
                fraction: 000,
            },
        },
        adf_state: AdfState {
            active_frequency: 123,
            standby_frequency: 123,
        },
        dme_state: DmeState { distance: 0.0 },
        xpdr_state: XpdrState {
            code: [1, 0, 0, 0],
            selected_digit: 0,
        },
        autopilot_state: AutopilotState {
            airspeed: 0,
            heading: 0,
            altitude: 100, // Airbus A320's minimum setting is 100, might as well initiate as such
            vertical_speed: 0,
            selected_setting: AutopilotValue::Altitude,
        },
    }
}
