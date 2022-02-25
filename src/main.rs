use std::convert::TryInto;

use crate::radio_device::SaitekRadioPanel;
use crate::radio_states::*;

mod radio_constants;
mod radio_device;
mod radio_display;
mod radio_states;

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

fn main() {
    let mut radio_panel = SaitekRadioPanel::new();
    show_standby_screen(&mut radio_panel);

    let mut mode_states_upper = ModeStates {
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

    loop {
        let input = radio_panel.wait_for_input();
        match input.mode_selector_upper {
            ModeSelectorState::ModeSelectorCom1 => {
                frequency_logic(
                    input,
                    &mut mode_states_upper.com1_state,
                    Window::TopLeft,
                    Window::TopRight,
                    &mut radio_panel,
                );
            }
            ModeSelectorState::ModeSelectorCom2 => {
                frequency_logic(
                    input,
                    &mut mode_states_upper.com2_state,
                    Window::TopLeft,
                    Window::TopRight,
                    &mut radio_panel,
                );
            }
            ModeSelectorState::ModeSelectorNav1 => {
                frequency_logic(
                    input,
                    &mut mode_states_upper.nav1_state,
                    Window::TopLeft,
                    Window::TopRight,
                    &mut radio_panel,
                );
            }
            ModeSelectorState::ModeSelectorNav2 => {
                frequency_logic(
                    input,
                    &mut mode_states_upper.nav2_state,
                    Window::TopLeft,
                    Window::TopRight,
                    &mut radio_panel,
                );
            }
            ModeSelectorState::ModeSelectorAdf => {
                frequency_logic(
                    input,
                    &mut mode_states_upper.adf_state,
                    Window::TopLeft,
                    Window::TopRight,
                    &mut radio_panel,
                );
            }
            ModeSelectorState::ModeSelectorDme => {
                dme_logic(&mut radio_panel, Window::TopLeft, Window::TopRight);
            }
            ModeSelectorState::ModeSelectorXpdr => {
                xpdr_logic(
                    input,
                    &mut mode_states_upper.xpdr_state,
                    Window::TopLeft,
                    Window::TopRight,
                    &mut radio_panel,
                );
            }
        }
    }
}

fn xpdr_logic(
    input: radio_device::InputState,
    xpdr_state: &mut XpdrState,
    window_active: Window,
    window_standby: Window,
    radio_panel: &mut SaitekRadioPanel,
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
        wrap(xpdr_state.code[xpdr_state.selected_digit], 0, 10);

    let code = xpdr_state.code.map(|d| d.to_string()).join("");
    let code = format!(" {}", code);
    let code: String = code
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i == (xpdr_state.selected_digit + 1) {
                format!("{}.", c)
            } else {
                c.to_string()
            }
        })
        .collect();

    radio_panel.set_window(window_active as usize, "     ");
    radio_panel.set_window(window_standby as usize, &code);
    radio_panel.update_all_displays();
}

fn dme_logic(radio_panel: &mut SaitekRadioPanel, window_active: Window, window_standby: Window) {
    radio_panel.set_window(window_active as usize, "   0.0");
    radio_panel.set_window(window_standby as usize, "    0");
    radio_panel.update_all_displays();
}

fn frequency_logic(
    input: radio_device::InputState,
    frequency_state: &mut FrequencyState,
    window_active: Window,
    window_standby: Window,
    radio_panel: &mut SaitekRadioPanel,
) {
    if matches!(input.button_upper, ButtonState::Pressed) {
        swap_frequencies(frequency_state);
    }

    // More consise variable names
    let active_whole = frequency_state.active_whole_part;
    let active_fract = frequency_state.active_fractional_part;
    let mut standby_whole = frequency_state.standby_whole_part;
    let mut standby_fract = frequency_state.standby_fractional_part;

    standby_whole += match input.rotary_upper_outer {
        RotaryState::Clockwise => 1,
        RotaryState::CounterClockwise => -1,
        RotaryState::None => 0,
    };
    standby_fract += match input.rotary_upper_inner {
        RotaryState::Clockwise => 5,
        RotaryState::CounterClockwise => -5,
        RotaryState::None => 0,
    };

    standby_whole = wrap(standby_whole, 118, 135);
    standby_fract = wrap(standby_fract, 0, 100);
    let active_frequency = format!("{:0>3}.{:0>2}", active_whole, active_fract);
    let standby_frequency = format!("{:0>3}.{:0>2}", standby_whole, standby_fract);
    radio_panel.set_window(window_active as usize, &active_frequency);
    radio_panel.set_window(window_standby as usize, &standby_frequency);
    radio_panel.update_all_displays();

    // Save values for next iteration
    frequency_state.standby_whole_part = standby_whole;
    frequency_state.standby_fractional_part = standby_fract;
    frequency_state.active_whole_part = active_whole;
    frequency_state.active_fractional_part = active_fract;
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
fn show_standby_screen(radio_panel: &mut SaitekRadioPanel) {
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
