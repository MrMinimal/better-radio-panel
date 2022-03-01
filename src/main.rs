use std::{mem::transmute_copy, fmt::Display};
use std::thread::sleep;
use std::time::Duration;

use parse_int::parse;
use simconnect::{self, SimConnector};
use std::convert::TryInto;

use radio_panel::{
    device::{InputState, RadioPanel},
    input_states::{ButtonState, ModeSelectorState, RotaryState},
};

mod radio_panel;

enum Window {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

struct FrequencyState {
    standby_whole_part: i16,
    standby_fractional_part: i16,
    active_whole_part: i16,
    active_fractional_part: i16,
}

struct DmeState {
    miles: f32,
    knots: i16,
}

struct XpdrState {
    code: [i8; 4],
    selected_digit: usize,
}

struct ModeStates {
    com1_state: FrequencyState,
    com2_state: FrequencyState,
    nav1_state: FrequencyState,
    nav2_state: FrequencyState,
    adf_state: FrequencyState,
    dme_state: DmeState,
    xpdr_state: XpdrState,
}

struct DataStruct {
    lat: f64,
    lon: f64,
    alt: f64,
}

fn main() {
    let mut simulator = simconnect::SimConnector::new();
    simulator.connect("Simple Program"); // Intialize connection with SimConnect

    let mut radio_panel = RadioPanel::new();
    show_standby_screen(&mut radio_panel);

    let mut mode_states = ModeStates {
        com1_state: FrequencyState {
            standby_whole_part: 118,
            standby_fractional_part: 0,
            active_whole_part: 118,
            active_fractional_part: 0,
        },
        com2_state: FrequencyState {
            standby_whole_part: 118,
            standby_fractional_part: 0,
            active_whole_part: 118,
            active_fractional_part: 0,
        },
        nav1_state: FrequencyState {
            standby_whole_part: 118,
            standby_fractional_part: 0,
            active_whole_part: 118,
            active_fractional_part: 0,
        },
        nav2_state: FrequencyState {
            standby_whole_part: 118,
            standby_fractional_part: 0,
            active_whole_part: 118,
            active_fractional_part: 0,
        },
        adf_state: FrequencyState {
            standby_whole_part: 118,
            standby_fractional_part: 0,
            active_whole_part: 118,
            active_fractional_part: 0,
        },
        dme_state: DmeState {
            miles: 0.0,
            knots: 0,
        },
        xpdr_state: XpdrState {
            code: [0; 4],
            selected_digit: 0,
        },
    };

    // Tell simulator which event ID is supposed to represent what simulator event
    simulator.map_client_event_to_sim_event(1000, "COM_RADIO_SET_HZ");
    simulator.map_client_event_to_sim_event(1001, "COM_STBY_RADIO_SET_HZ");
    simulator.map_client_event_to_sim_event(1002, "COM2_RADIO_SET_HZ");
    simulator.map_client_event_to_sim_event(1003, "COM2_STBY_RADIO_SET_HZ");
    simulator.map_client_event_to_sim_event(1004, "NAV1_RADIO_SET_HZ");
    simulator.map_client_event_to_sim_event(1005, "NAV1_STBY_SET_HZ");
    simulator.map_client_event_to_sim_event(1006, "NAV2_RADIO_SET_HZ");
    simulator.map_client_event_to_sim_event(1007, "NAV2_STBY_SET_HZ");
    simulator.map_client_event_to_sim_event(1008, "XPNDR_SET");

    loop {
        let input = radio_panel.wait_for_input();

        match input.mode_selector_upper {
            ModeSelectorState::ModeSelectorCom1 => {
                handle_inputs(
                    &mut mode_states.com1_state,
                    input.button_upper,
                    input.rotary_upper_outer,
                    input.rotary_upper_inner,
                );
                display_values(
                    &mut mode_states.com1_state,
                    Window::TopLeft,
                    Window::TopRight,
                    &mut radio_panel,
                    &simulator,
                    1000,
                    1001,
                );
            }
            ModeSelectorState::ModeSelectorCom2 => {
                handle_inputs(
                    &mut mode_states.com2_state,
                    input.button_upper,
                    input.rotary_upper_outer,
                    input.rotary_upper_inner,
                );
                display_values(
                    &mut mode_states.com2_state,
                    Window::TopLeft,
                    Window::TopRight,
                    &mut radio_panel,
                    &simulator,
                    1002,
                    1003,
                );
            }
            ModeSelectorState::ModeSelectorNav1 => {
                handle_inputs(
                    &mut mode_states.nav1_state,
                    input.button_upper,
                    input.rotary_upper_outer,
                    input.rotary_upper_inner,
                );
                display_values(
                    &mut mode_states.nav1_state,
                    Window::TopLeft,
                    Window::TopRight,
                    &mut radio_panel,
                    &simulator,
                    1000,
                    1000,
                );
            }
            ModeSelectorState::ModeSelectorNav2 => {
                handle_inputs(
                    &mut mode_states.nav2_state,
                    input.button_upper,
                    input.rotary_upper_outer,
                    input.rotary_upper_inner,
                );
                display_values(
                    &mut mode_states.nav2_state,
                    Window::TopLeft,
                    Window::TopRight,
                    &mut radio_panel,
                    &simulator,
                    1000,
                    1000,
                );
            }
            ModeSelectorState::ModeSelectorAdf => {
                handle_inputs(
                    &mut mode_states.adf_state,
                    input.button_upper,
                    input.rotary_upper_outer,
                    input.rotary_upper_inner,
                );
                display_values(
                    &mut mode_states.adf_state,
                    Window::TopLeft,
                    Window::TopRight,
                    &mut radio_panel,
                    &simulator,
                    1000,
                    1000,
                );
            }
            ModeSelectorState::ModeSelectorDme => {
                dme_logic(&mut radio_panel, Window::TopLeft, Window::TopRight);
            }
            ModeSelectorState::ModeSelectorXpdr => {
                xpdr_logic(
                    input,
                    &mut mode_states.xpdr_state,
                    Window::TopLeft,
                    Window::TopRight,
                    &mut radio_panel,
                    &simulator,
                );
            }
        }

        match input.mode_selector_lower {
            ModeSelectorState::ModeSelectorCom1 => {
                handle_inputs(
                    &mut mode_states.com1_state,
                    input.button_lower,
                    input.rotary_lower_outer,
                    input.rotary_lower_inner,
                );
                display_values(
                    &mut mode_states.com1_state,
                    Window::BottomLeft,
                    Window::BottomRight,
                    &mut radio_panel,
                    &simulator,
                    1000,
                    1001,
                );
            }
            ModeSelectorState::ModeSelectorCom2 => {
                handle_inputs(
                    &mut mode_states.com2_state,
                    input.button_lower,
                    input.rotary_lower_outer,
                    input.rotary_lower_inner,
                );
                display_values(
                    &mut mode_states.com2_state,
                    Window::BottomLeft,
                    Window::BottomRight,
                    &mut radio_panel,
                    &simulator,
                    1002,
                    1003,
                );
            }
            ModeSelectorState::ModeSelectorNav1 => {
                handle_inputs(
                    &mut mode_states.nav1_state,
                    input.button_lower,
                    input.rotary_lower_outer,
                    input.rotary_lower_inner,
                );
                display_values(
                    &mut mode_states.nav1_state,
                    Window::BottomLeft,
                    Window::BottomRight,
                    &mut radio_panel,
                    &simulator,
                    1004,
                    1005,
                );
            }
            ModeSelectorState::ModeSelectorNav2 => {
                handle_inputs(
                    &mut mode_states.nav2_state,
                    input.button_lower,
                    input.rotary_lower_outer,
                    input.rotary_lower_inner,
                );
                display_values(
                    &mut mode_states.nav2_state,
                    Window::BottomLeft,
                    Window::BottomRight,
                    &mut radio_panel,
                    &simulator,
                    1006,
                    1007,
                );
            }
            ModeSelectorState::ModeSelectorAdf => {
                handle_inputs(
                    &mut mode_states.adf_state,
                    input.button_lower,
                    input.rotary_lower_outer,
                    input.rotary_lower_inner,
                );
                display_values(
                    &mut mode_states.adf_state,
                    Window::BottomLeft,
                    Window::BottomRight,
                    &mut radio_panel,
                    &simulator,
                    1000,
                    1000,
                );
            }
            ModeSelectorState::ModeSelectorDme => {
                dme_logic(&mut radio_panel, Window::BottomLeft, Window::BottomRight);
            }
            ModeSelectorState::ModeSelectorXpdr => {
                xpdr_logic(
                    input,
                    &mut mode_states.xpdr_state,
                    Window::BottomLeft,
                    Window::BottomRight,
                    &mut radio_panel,
                    &simulator,
                );
            }
        }
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

    radio_panel.set_window(window_active as usize, "     ");
    radio_panel.set_window(window_standby as usize, &code);
    radio_panel.update_all_displays();
}

fn dme_logic(radio_panel: &mut RadioPanel, window_active: Window, window_standby: Window) {
    radio_panel.set_window(window_active as usize, "   0.0");
    radio_panel.set_window(window_standby as usize, "    0");
    radio_panel.update_all_displays();
}

fn handle_inputs(
    frequency_state: &mut FrequencyState,
    swap_button: ButtonState,
    outer_rotary: RotaryState,
    inner_rotary: RotaryState,
) {
    if matches!(swap_button, ButtonState::Pressed) {
        swap_frequencies(frequency_state);
    }

    // More consise variable names
    let active_whole = frequency_state.active_whole_part;
    let active_fract = frequency_state.active_fractional_part;
    let mut standby_whole = frequency_state.standby_whole_part;
    let mut standby_fract = frequency_state.standby_fractional_part;

    standby_whole += match outer_rotary {
        RotaryState::Clockwise => 1,
        RotaryState::CounterClockwise => -1,
        RotaryState::None => 0,
    };
    standby_fract += match inner_rotary {
        RotaryState::Clockwise => 5,
        RotaryState::CounterClockwise => -5,
        RotaryState::None => 0,
    };

    standby_whole = wrap(standby_whole, 118, 137);
    standby_fract = wrap(standby_fract, 0, 1000);

    // Save values for next iteration
    frequency_state.active_whole_part = active_whole;
    frequency_state.active_fractional_part = active_fract;
    frequency_state.standby_whole_part = standby_whole;
    frequency_state.standby_fractional_part = standby_fract;

}

fn display_values(
    frequency_state: &mut FrequencyState,
    window_active: Window,
    window_standby: Window,
    radio_panel: &mut RadioPanel,
    simulator: &SimConnector,
    active_event_id: u32,
    standby_event_id: u32,
) {
    // More consise variable names
    let active_whole = frequency_state.active_whole_part;
    let active_fract = frequency_state.active_fractional_part;
    let mut standby_whole = frequency_state.standby_whole_part;
    let mut standby_fract = frequency_state.standby_fractional_part;

    // Format for FS2020
    let active_whole = format!("{:0>3}", active_whole);
    let active_fract = format!("{:0>3}", active_fract);
    let active_frequency = format!("{}{}000", active_whole, active_fract);
    let active_frequency = parse::<u32>(&active_frequency).unwrap();
    let standby_whole = format!("{:0>3}", standby_whole);
    let standby_fract = format!("{:0>3}", standby_fract);
    let standby_frequency = format!("{}{}000", standby_whole, standby_fract);
    let standby_frequency = parse::<u32>(&standby_frequency).unwrap();
    simulator.transmit_client_event(1, active_event_id, active_frequency, 5, 0);
    simulator.transmit_client_event(1, standby_event_id, standby_frequency, 5, 0);

    // Format for hardware
    let active_whole = active_whole.to_string()[1..].to_string(); // truncate first digit because display can only display 5
    let active_frequency = format!("{}.{}", active_whole, active_fract);
    let standby_whole = standby_whole.to_string()[1..].to_string(); // truncate first digit because display can only display 5
    let standby_frequency = format!("{}.{}", standby_whole, standby_fract);
    radio_panel.set_window(window_active as usize, &active_frequency);
    radio_panel.set_window(window_standby as usize, &standby_frequency);
    radio_panel.update_all_displays();
}

fn swap_frequencies(frequency_state: &mut FrequencyState) {
    let previous_active_whole = frequency_state.active_whole_part;
    let previous_active_fract = frequency_state.active_fractional_part;
    frequency_state.active_whole_part = frequency_state.standby_whole_part;
    frequency_state.active_fractional_part = frequency_state.standby_fractional_part;
    frequency_state.standby_whole_part = previous_active_whole;
    frequency_state.standby_fractional_part = previous_active_fract;
}

/// Show only dashes to indicate no data recieved from sim yet
fn show_standby_screen(radio_panel: &mut RadioPanel) {
    radio_panel.set_window(0, "    -");
    radio_panel.set_window(1, "    -");
    radio_panel.set_window(2, "    -");
    radio_panel.set_window(3, "    -");
    radio_panel.update_all_displays();
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
