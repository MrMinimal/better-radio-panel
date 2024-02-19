use parse_int::parse;
use radio_panel::{
    constants::*,
    device::{InputState, RadioPanel},
    frequency::Frequency,
    input_states::{ButtonState, ModeSelectorState, RotaryState},
    states::*,
    windows::*,
};
use simconnect::{self, SimConnector};
use std::{
    thread,
    time::{self},
};

mod radio_panel;

pub struct AutopilotState {
    pub airspeed: i16,
    pub heading: i16,
    pub altitude: i32,
    pub vertical_speed: i16,
    selected_setting: AutopilotValue,
}

enum AutopilotValue {
    Altitude,
    VerticalSpeed,
}

struct PlaneState {
    com1_state: FrequencyState,
    com2_state: FrequencyState,
    nav1_state: FrequencyState,
    nav2_state: FrequencyState,
    adf_state: AdfState,
    dme_state: DmeState,
    xpdr_state: XpdrState,
    autopilot_state: AutopilotState,
}

fn main() {
    let mut radio_panel = RadioPanel::new();
    let mut state = plane_default_state();
    let mut simulator = simconnect::SimConnector::new();
    let mut connected_to_sim = false;

    let mut input = InputState::new();
    loop {
        if connected_to_sim {
            input = match radio_panel.block_until_input() {
                Some(updated_input_state) => updated_input_state,
                None => input,
            };

            if input.mode_selector_upper == input.mode_selector_lower {
                handle_autopilot_input(
                    &mut state.autopilot_state,
                    input.rotary_upper_outer,
                    input.rotary_upper_inner,
                    input.rotary_lower_outer,
                    input.rotary_lower_inner,
                    input.button_lower,
                );
                autopilot_logic(&state.autopilot_state, &simulator, &mut radio_panel);
                continue;
            }

            match input.mode_selector_upper {
                ModeSelectorState::ModeSelectorCom1 => {
                    handle_com_frequency_input(
                        &mut state.com1_state,
                        input.button_upper,
                        input.rotary_upper_outer,
                        input.rotary_upper_inner,
                    );
                    connected_to_sim = display_values(
                        3,
                        &mut state.com1_state,
                        Window::TopLeft,
                        Window::TopRight,
                        &mut radio_panel,
                        &simulator,
                        EVENT_ID_COM_RADIO_SET_HZ,
                        EVENT_ID_COM_STBY_RADIO_SET_HZ,
                    );
                }
                ModeSelectorState::ModeSelectorCom2 => {
                    handle_com_frequency_input(
                        &mut state.com2_state,
                        input.button_upper,
                        input.rotary_upper_outer,
                        input.rotary_upper_inner,
                    );
                    connected_to_sim = display_values(
                        3,
                        &mut state.com2_state,
                        Window::TopLeft,
                        Window::TopRight,
                        &mut radio_panel,
                        &simulator,
                        EVENT_ID_COM2_RADIO_SET_HZ,
                        EVENT_ID_COM2_STBY_RADIO_SET_HZ,
                    );
                }
                ModeSelectorState::ModeSelectorNav1 => {
                    handle_nav_frequency_input(
                        &mut state.nav1_state,
                        input.button_upper,
                        input.rotary_upper_outer,
                        input.rotary_upper_inner,
                    );
                    connected_to_sim = display_nav_values(
                        &mut state.nav1_state,
                        Window::TopLeft,
                        Window::TopRight,
                        &mut radio_panel,
                    );
                }
                ModeSelectorState::ModeSelectorNav2 => {
                    handle_nav_frequency_input(
                        &mut state.nav2_state,
                        input.button_upper,
                        input.rotary_upper_outer,
                        input.rotary_upper_inner,
                    );
                    connected_to_sim = display_nav_values(
                        &mut state.nav2_state,
                        Window::TopLeft,
                        Window::TopRight,
                        &mut radio_panel,
                    );
                }
                ModeSelectorState::ModeSelectorAdf => {
                    connected_to_sim = display_adf_values(
                        &mut state.adf_state,
                        Window::TopLeft,
                        Window::TopRight,
                        &mut radio_panel,
                    );
                }
                ModeSelectorState::ModeSelectorDme => {
                    dme_logic(
                        &mut radio_panel,
                        &mut state.dme_state,
                        Window::TopLeft,
                        Window::TopRight,
                    );
                }
                ModeSelectorState::ModeSelectorXpdr => {
                    xpdr_logic(
                        input,
                        &mut state.xpdr_state,
                        Window::TopLeft,
                        Window::TopRight,
                        &mut radio_panel,
                        &simulator,
                    );
                }
            }

            match input.mode_selector_lower {
                ModeSelectorState::ModeSelectorCom1 => {
                    handle_com_frequency_input(
                        &mut state.com1_state,
                        input.button_lower,
                        input.rotary_lower_outer,
                        input.rotary_lower_inner,
                    );
                    connected_to_sim = display_values(
                        3,
                        &mut state.com1_state,
                        Window::BottomLeft,
                        Window::BottomRight,
                        &mut radio_panel,
                        &simulator,
                        EVENT_ID_COM_RADIO_SET_HZ,
                        EVENT_ID_COM_STBY_RADIO_SET_HZ,
                    );
                }
                ModeSelectorState::ModeSelectorCom2 => {
                    handle_com_frequency_input(
                        &mut state.com2_state,
                        input.button_lower,
                        input.rotary_lower_outer,
                        input.rotary_lower_inner,
                    );
                    connected_to_sim = display_values(
                        3,
                        &mut state.com2_state,
                        Window::BottomLeft,
                        Window::BottomRight,
                        &mut radio_panel,
                        &simulator,
                        EVENT_ID_COM2_RADIO_SET_HZ,
                        EVENT_ID_COM2_STBY_RADIO_SET_HZ,
                    );
                }
                ModeSelectorState::ModeSelectorNav1 => {
                    handle_nav_frequency_input(
                        &mut state.nav1_state,
                        input.button_lower,
                        input.rotary_lower_outer,
                        input.rotary_lower_inner,
                    );
                    connected_to_sim = display_nav_values(
                        &mut state.nav1_state,
                        Window::BottomLeft,
                        Window::BottomRight,
                        &mut radio_panel,
                    );
                }
                ModeSelectorState::ModeSelectorNav2 => {
                    handle_nav_frequency_input(
                        &mut state.nav2_state,
                        input.button_lower,
                        input.rotary_lower_outer,
                        input.rotary_lower_inner,
                    );
                    connected_to_sim = display_nav_values(
                        &mut state.nav2_state,
                        Window::BottomLeft,
                        Window::BottomRight,
                        &mut radio_panel,
                    );
                }
                ModeSelectorState::ModeSelectorAdf => {
                    connected_to_sim = display_adf_values(
                        &mut state.adf_state,
                        Window::BottomLeft,
                        Window::BottomRight,
                        &mut radio_panel,
                    );
                }
                ModeSelectorState::ModeSelectorDme => {
                    dme_logic(
                        &mut radio_panel,
                        &state.dme_state,
                        Window::BottomLeft,
                        Window::BottomRight,
                    );
                }
                ModeSelectorState::ModeSelectorXpdr => {
                    xpdr_logic(
                        input,
                        &mut state.xpdr_state,
                        Window::BottomLeft,
                        Window::BottomRight,
                        &mut radio_panel,
                        &simulator,
                    );
                }
            }
        } else {
            loop {
                if simulator.connect("BetterRadioPanel") {
                    setup_simulator_event_ids(&mut simulator);
                    connected_to_sim = true;
                    show_standby_screen(&mut radio_panel);
                    break;
                } else {
                    show_connecting_animation(&mut radio_panel)
                }
            }
        }
    }
}

fn setup_simulator_event_ids(simulator: &mut SimConnector) {
    // Tell simulator which event ID is supposed to represent what simulator event
    simulator.map_client_event_to_sim_event(EVENT_ID_COM_RADIO_SET_HZ, "COM_RADIO_SET_HZ");
    simulator
        .map_client_event_to_sim_event(EVENT_ID_COM_STBY_RADIO_SET_HZ, "COM_STBY_RADIO_SET_HZ");
    simulator.map_client_event_to_sim_event(EVENT_ID_COM2_RADIO_SET_HZ, "COM2_RADIO_SET_HZ");
    simulator
        .map_client_event_to_sim_event(EVENT_ID_COM2_STBY_RADIO_SET_HZ, "COM2_STBY_RADIO_SET_HZ");
    simulator.map_client_event_to_sim_event(EVENT_ID_NAV1_RADIO_SET_HZ, "NAV1_RADIO_SET_HZ");
    simulator.map_client_event_to_sim_event(EVENT_ID_NAV1_STBY_SET_HZ, "NAV1_STBY_SET_HZ");
    simulator.map_client_event_to_sim_event(EVENT_ID_NAV2_RADIO_SET_HZ, "NAV2_RADIO_SET_HZ");
    simulator.map_client_event_to_sim_event(EVENT_ID_NAV2_STBY_SET_HZ, "NAV2_STBY_SET_HZ");
    simulator.map_client_event_to_sim_event(EVENT_ID_XPNDR_SET, "XPNDR_SET");

    simulator.map_client_event_to_sim_event(EVENT_ID_HEADING_BUG_SET, "HEADING_BUG_SET");
    simulator
        .map_client_event_to_sim_event(EVENT_ID_AP_ALT_VAR_SET_ENGLISH, "AP_ALT_VAR_SET_ENGLISH");
    simulator
        .map_client_event_to_sim_event(EVENT_ID_AP_VS_VAR_SET_ENGLISH, "AP_VS_VAR_SET_ENGLISH");
    simulator.map_client_event_to_sim_event(EVENT_ID_AP_SPD_VAR_SET, "AP_SPD_VAR_SET");
}

fn plane_default_state() -> PlaneState {
    PlaneState {
        com1_state: FrequencyState {
            standby_freq: Frequency {
                integer: 118,
                fractional: 000,
            },
            active_freq: Frequency {
                integer: 118,
                fractional: 000,
            },
        },
        com2_state: FrequencyState {
            standby_freq: Frequency {
                integer: 118,
                fractional: 000,
            },
            active_freq: Frequency {
                integer: 118,
                fractional: 000,
            },
        },
        nav1_state: FrequencyState {
            standby_freq: Frequency {
                integer: 108,
                fractional: 000,
            },
            active_freq: Frequency {
                integer: 108,
                fractional: 000,
            },
        },
        nav2_state: FrequencyState {
            standby_freq: Frequency {
                integer: 108,
                fractional: 000,
            },
            active_freq: Frequency {
                integer: 108,
                fractional: 000,
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

fn show_connecting_animation(radio_panel: &mut RadioPanel) {
    radio_panel.clear_all_windows();

    for window_index in 0..4 {
        let window = match window_index {
            0 => Window::TopLeft,
            1 => Window::TopRight,
            2 => Window::BottomLeft,
            3 => Window::BottomRight,
            _ => Window::TopLeft,
        };

        for character_index in 0..5 {
            let mut content = String::from("     ");
            content.replace_range(character_index..character_index + 1, "-");

            radio_panel.set_window(window, &content);
            radio_panel.update_all_windows();

            thread::sleep(time::Duration::from_millis(300));
        }

        radio_panel.set_window(window, "     ")
    }
}

fn xpdr_logic(
    input: InputState,
    xpdr_state: &mut XpdrState,
    window_active: Window,
    window_standby: Window,
    radio_panel: &mut RadioPanel,
    simulator: &SimConnector,
) {
    if matches!(input.button_upper, ButtonState::Pressed) {
        xpdr_state.selected_digit += 1;
        xpdr_state.selected_digit =
            wrap(xpdr_state.selected_digit.try_into().unwrap(), 0, 4) as usize;
    }

    xpdr_state.code[xpdr_state.selected_digit] += match input.rotary_upper_outer {
        RotaryState::Clockwise => 1,
        RotaryState::CounterClockwise => -1,
        RotaryState::None => 0,
    };
    xpdr_state.code[xpdr_state.selected_digit] += match input.rotary_upper_inner {
        RotaryState::Clockwise => 1,
        RotaryState::CounterClockwise => -1,
        RotaryState::None => 0,
    };
    xpdr_state.code[xpdr_state.selected_digit] =
        wrap(xpdr_state.code[xpdr_state.selected_digit], 0, 8);

    let code = xpdr_state.code.map(|d| d.to_string()).join("");
    let hex = format!("0x{}", code);
    let hex = parse::<u32>(&hex).unwrap();
    let code = format!(" {}", code);
    let code: String = code
        .chars()
        .enumerate()
        .map(|(i, char)| {
            if i == (xpdr_state.selected_digit + 1) {
                format!("{}.", char) // if digit selected, add dot to it
            } else {
                char.to_string()
            }
        })
        .collect();

    simulator.transmit_client_event(1, 1008, hex, 5, 0);

    radio_panel.set_window(window_active, "     ");
    radio_panel.set_window(window_standby, &code);
    radio_panel.update_all_windows();
}

fn dme_logic(
    radio_panel: &mut RadioPanel,
    dme_state: &DmeState,
    window_active: Window,
    window_standby: Window,
) {
    let formatted_distance = format!("   {:.1}", dme_state.distance);
    radio_panel.set_window(window_active, "     ");
    radio_panel.set_window(window_standby, &formatted_distance);
    radio_panel.update_all_windows();
}

fn autopilot_logic(state: &AutopilotState, simulator: &SimConnector, radio_panel: &mut RadioPanel) {
    simulator.transmit_client_event(1, 1009, state.heading as u32, 5, 0);
    simulator.transmit_client_event(1, 1010, state.altitude as u32, 5, 0);
    simulator.transmit_client_event(1, 1011, state.vertical_speed as u32, 5, 0);
    simulator.transmit_client_event(1, 1012, state.airspeed as u32, 5, 0);
    radio_panel.set_window(Window::TopLeft, &format!("{:>5}", state.airspeed));
    radio_panel.set_window(Window::TopRight, &format!("  {:0>3}", state.heading));

    let selected_indicator_altitude = match state.selected_setting {
        AutopilotValue::Altitude => ".",
        AutopilotValue::VerticalSpeed => "",
    };

    let selected_indicator_vertical_speed = match state.selected_setting {
        AutopilotValue::Altitude => "",
        AutopilotValue::VerticalSpeed => ".",
    };

    radio_panel.set_window(
        Window::BottomLeft,
        &format!("{:0>5}{}", state.altitude, selected_indicator_altitude),
    );

    // make sure formatting is the same as in an Airbus (align right, always display 4 digits, sign in front)
    match state.vertical_speed {
        s if s >= 0 => radio_panel.set_window(
            Window::BottomRight,
            &format!(
                " {:0>4}{}",
                state.vertical_speed, selected_indicator_vertical_speed
            ),
        ),
        s if s < 0 => radio_panel.set_window(
            Window::BottomRight,
            &format!(
                "{:05}{}",
                state.vertical_speed, selected_indicator_vertical_speed
            ),
        ),
        _ => (),
    }
    radio_panel.update_all_windows();
}

fn handle_com_frequency_input(
    frequency_state: &mut FrequencyState,
    swap_button: ButtonState,
    outer_rotary: RotaryState,
    inner_rotary: RotaryState,
) {
    if matches!(swap_button, ButtonState::Pressed) {
        swap_frequencies(frequency_state);
    }

    frequency_state.standby_freq.integer += match outer_rotary {
        RotaryState::Clockwise => 1,
        RotaryState::CounterClockwise => -1,
        RotaryState::None => 0,
    };
    frequency_state.standby_freq.fractional += match inner_rotary {
        RotaryState::Clockwise => 5,
        RotaryState::CounterClockwise => -5,
        RotaryState::None => 0,
    };

    frequency_state.standby_freq.integer = wrap(frequency_state.standby_freq.integer, 118, 137);
    frequency_state.standby_freq.fractional =
        wrap(frequency_state.standby_freq.fractional, 0, 1000);
}

fn handle_nav_frequency_input(
    frequency_state: &mut FrequencyState,
    swap_button: ButtonState,
    outer_rotary: RotaryState,
    inner_rotary: RotaryState,
) {
    if matches!(swap_button, ButtonState::Pressed) {
        swap_frequencies(frequency_state);
    }

    frequency_state.standby_freq.integer += match outer_rotary {
        RotaryState::Clockwise => 1,
        RotaryState::CounterClockwise => -1,
        RotaryState::None => 0,
    };
    frequency_state.standby_freq.fractional += match inner_rotary {
        RotaryState::Clockwise => 50,
        RotaryState::CounterClockwise => -50,
        RotaryState::None => 0,
    };

    frequency_state.standby_freq.integer = wrap(frequency_state.standby_freq.integer, 108, 118);
    frequency_state.standby_freq.fractional =
        wrap(frequency_state.standby_freq.fractional, 0, 1000);
}

fn handle_autopilot_input(
    autopilot_state: &mut AutopilotState,
    outer_rotary_upper: RotaryState,
    inner_rotary_upper: RotaryState,
    outer_rotary_lower: RotaryState,
    inner_rotary_lower: RotaryState,
    select_button: ButtonState,
) {
    if matches!(select_button, ButtonState::Pressed) {
        autopilot_state.selected_setting = match autopilot_state.selected_setting {
            AutopilotValue::Altitude => AutopilotValue::VerticalSpeed,
            AutopilotValue::VerticalSpeed => AutopilotValue::Altitude,
        }
    }

    autopilot_state.heading += match outer_rotary_upper {
        RotaryState::Clockwise => 1,
        RotaryState::CounterClockwise => -1,
        RotaryState::None => 0,
    };
    autopilot_state.heading = wrap(autopilot_state.heading, 0, 360);

    autopilot_state.airspeed += match inner_rotary_upper {
        RotaryState::Clockwise => 1,
        RotaryState::CounterClockwise => -1,
        RotaryState::None => 0,
    };
    autopilot_state.airspeed = autopilot_state.airspeed.clamp(0, 9999);

    match autopilot_state.selected_setting {
        AutopilotValue::Altitude => {
            autopilot_state.altitude += match outer_rotary_lower {
                RotaryState::Clockwise => 1000,
                RotaryState::CounterClockwise => -1000,
                RotaryState::None => 0,
            };

            autopilot_state.altitude += match inner_rotary_lower {
                RotaryState::Clockwise => 100,
                RotaryState::CounterClockwise => -100,
                RotaryState::None => 0,
            };
            autopilot_state.altitude = autopilot_state.altitude.clamp(100, 100000);
        }
        AutopilotValue::VerticalSpeed => {
            // altitude
            autopilot_state.vertical_speed += match outer_rotary_lower {
                RotaryState::Clockwise => 100,
                RotaryState::CounterClockwise => -100,
                RotaryState::None => 0,
            };

            autopilot_state.vertical_speed += match inner_rotary_lower {
                RotaryState::Clockwise => 100,
                RotaryState::CounterClockwise => -100,
                RotaryState::None => 0,
            };
            autopilot_state.vertical_speed = autopilot_state.vertical_speed.clamp(-9900, 9900);
        }
    }
}

fn display_values(
    _fractional_digits: u8,
    frequency_state: &mut FrequencyState,
    window_active: Window,
    window_standby: Window,
    radio_panel: &mut RadioPanel,
    simulator: &SimConnector,
    active_event_id: u32,
    standby_event_id: u32,
) -> bool {
    // More consise variable names
    let active_integer = frequency_state.active_freq.integer;
    let active_fract = frequency_state.active_freq.fractional;
    let standby_integer = frequency_state.standby_freq.integer;
    let standby_fract = frequency_state.standby_freq.fractional;

    // Format for FS2020
    let active_integer = format!("{:0>3}", active_integer);
    let active_fract = format!("{:0>3}", active_fract);
    let active_frequency = format!("{}{}000", active_integer, active_fract);
    let active_frequency = parse::<u32>(&active_frequency).unwrap();
    let standby_integer = format!("{:0>3}", standby_integer);
    let standby_fract = format!("{:0>3}", standby_fract);
    let standby_frequency = format!("{}{}000", standby_integer, standby_fract);
    let standby_frequency = parse::<u32>(&standby_frequency).unwrap();
    if !simulator.transmit_client_event(1, active_event_id, active_frequency, 5, 0) {
        return false;
    };
    if !simulator.transmit_client_event(1, standby_event_id, standby_frequency, 5, 0) {
        return false;
    }

    // Format for hardware
    let active_integer = active_integer.to_string()[1..].to_string(); // truncate first digit because display can only display 5
    let active_frequency = format!("{}.{}", active_integer, active_fract);
    let standby_integer = standby_integer.to_string()[1..].to_string(); // truncate first digit because display can only display 5
    let standby_frequency = format!("{}.{}", standby_integer, standby_fract);
    radio_panel.set_window(window_active, &active_frequency);
    radio_panel.set_window(window_standby, &standby_frequency);
    radio_panel.update_all_windows();

    return true;
}

fn display_nav_values(
    frequency_state: &mut FrequencyState,
    window_active: Window,
    window_standby: Window,
    radio_panel: &mut RadioPanel,
) -> bool {
    // More consise variable names
    let active_integer = frequency_state.active_freq.integer;
    let active_fract = frequency_state.active_freq.fractional;
    let standby_integer = frequency_state.standby_freq.integer;
    let standby_fract = frequency_state.standby_freq.fractional;

    let mut active_fract = format!("{:03}", active_fract);
    active_fract.truncate(2);
    radio_panel.set_window(
        window_active,
        &format!("{}.{}", active_integer, active_fract),
    );

    let mut standby_fract = format!("{:03}", standby_fract);
    standby_fract.truncate(2);
    radio_panel.set_window(
        window_standby,
        &format!("{}.{}", standby_integer, standby_fract),
    );

    radio_panel.update_all_windows();

    return true;
}

fn display_adf_values(
    adf_state: &mut AdfState,
    window_active: Window,
    window_standby: Window,
    radio_panel: &mut RadioPanel,
) -> bool {
    radio_panel.set_window(
        window_active,
        &format!("{: >5}", adf_state.active_frequency),
    );

    radio_panel.set_window(
        window_standby,
        &format!("{: >5}", adf_state.standby_frequency),
    );

    radio_panel.update_all_windows();

    return true;
}

fn swap_frequencies(frequency_state: &mut FrequencyState) {
    let previous_active_freq = frequency_state.active_freq;
    frequency_state.active_freq = frequency_state.standby_freq;
    frequency_state.standby_freq = previous_active_freq;
}

/// Show only dashes to indicate no data recieved from sim yet
fn show_standby_screen(radio_panel: &mut RadioPanel) {
    for window_index in 0..4 {
        let window = match window_index {
            0 => Window::TopLeft,
            1 => Window::TopRight,
            2 => Window::BottomLeft,
            3 => Window::BottomRight,
            _ => Window::TopLeft,
        };
        radio_panel.set_window(window, "-----");
    }
    radio_panel.update_all_windows();
}

/// Make sure values stay within min and max
/// Wraps around on both ends
fn wrap<T: std::cmp::PartialOrd + std::ops::Sub<Output = T> + std::ops::Add<Output = T>>(
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
