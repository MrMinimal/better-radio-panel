use parse_int::parse;
use radio_panel::{constants::*, device::*, frequency::*, hardware::*, states::*, utility::*};
use simconnect::{self, SimConnector};
use std::{
    thread,
    time::{self},
};

mod radio_panel;

fn main() {
    let mut radio_panel = RadioPanel::new();
    let mut state = instruments_default_state();
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
                apply_autopilot_input(
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

            handle_upper_panel(
                input,
                &mut state,
                &mut radio_panel,
                &mut connected_to_sim,
                &simulator,
            );
            handle_lower_panel(
                input,
                &mut state,
                &mut radio_panel,
                &mut connected_to_sim,
                &simulator,
            );
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

fn handle_upper_panel(
    input: InputState,
    state: &mut InstrumentStates,
    radio_panel: &mut RadioPanel,
    connected_to_sim: &mut bool,
    simulator: &SimConnector,
) {
    match input.mode_selector_upper {
        ModeSelectorState::ModeSelectorCom1 => {
            apply_com_input(
                &mut state.com1_state,
                input.button_upper,
                input.rotary_upper_outer,
                input.rotary_upper_inner,
            );
            display_frequency_on_hardware(
                radio_panel,
                &state.com1_state,
                Window::TopLeft,
                Window::TopRight,
                3,
            );
            *connected_to_sim = send_com_to_sim(
                &mut state.com1_state,
                simulator,
                EVENT_ID_COM_RADIO_SET_HZ,
                EVENT_ID_COM_STBY_RADIO_SET_HZ,
            );
        }
        ModeSelectorState::ModeSelectorCom2 => {
            apply_com_input(
                &mut state.com2_state,
                input.button_upper,
                input.rotary_upper_outer,
                input.rotary_upper_inner,
            );
            display_frequency_on_hardware(
                radio_panel,
                &state.com2_state,
                Window::TopLeft,
                Window::TopRight,
                3,
            );
            *connected_to_sim = send_com_to_sim(
                &mut state.com2_state,
                simulator,
                EVENT_ID_COM2_RADIO_SET_HZ,
                EVENT_ID_COM2_STBY_RADIO_SET_HZ,
            );
        }
        ModeSelectorState::ModeSelectorNav1 => {
            apply_nav_input(
                &mut state.nav1_state,
                input.button_upper,
                input.rotary_upper_outer,
                input.rotary_upper_inner,
            );
            display_frequency_on_hardware(
                radio_panel,
                &state.nav1_state,
                Window::TopLeft,
                Window::TopRight,
                2,
            );
            *connected_to_sim = send_nav_to_sim(
                &mut state.nav1_state,
                simulator,
                EVENT_ID_NAV1_RADIO_SET_HZ,
                EVENT_ID_NAV1_STBY_SET_HZ,
            );
        }
        ModeSelectorState::ModeSelectorNav2 => {
            apply_nav_input(
                &mut state.nav2_state,
                input.button_upper,
                input.rotary_upper_outer,
                input.rotary_upper_inner,
            );
            display_frequency_on_hardware(
                radio_panel,
                &state.nav2_state,
                Window::TopLeft,
                Window::TopRight,
                2,
            );
            *connected_to_sim = send_nav_to_sim(
                &mut state.nav1_state,
                simulator,
                EVENT_ID_NAV2_RADIO_SET_HZ,
                EVENT_ID_NAV2_STBY_SET_HZ,
            );
        }
        ModeSelectorState::ModeSelectorAdf => {
            display_adf_values(
                &state.adf_state,
                Window::TopLeft,
                Window::TopRight,
                radio_panel,
            );
        }
        ModeSelectorState::ModeSelectorDme => {
            display_dme_on_hardware(
                radio_panel,
                &state.dme_state,
                &state.nav1_state,
                Window::TopLeft,
                Window::TopRight,
            );
        }
        ModeSelectorState::ModeSelectorXpdr => {
            apply_xpdr_input(
                &mut state.xpdr_state,
                input.button_upper,
                input.rotary_upper_outer,
                input.rotary_upper_inner,
            );
            display_xpdr_on_hardware(
                radio_panel,
                &state.xpdr_state,
                Window::TopLeft,
                Window::TopRight,
            );
        }
    }
}

fn handle_lower_panel(
    input: InputState,
    state: &mut InstrumentStates,
    radio_panel: &mut RadioPanel,
    connected_to_sim: &mut bool,
    simulator: &SimConnector,
) {
    match input.mode_selector_lower {
        ModeSelectorState::ModeSelectorCom1 => {
            apply_com_input(
                &mut state.com1_state,
                input.button_lower,
                input.rotary_lower_outer,
                input.rotary_lower_inner,
            );
            display_frequency_on_hardware(
                radio_panel,
                &state.com1_state,
                Window::BottomLeft,
                Window::BottomRight,
                3,
            );
            *connected_to_sim = send_com_to_sim(
                &mut state.com1_state,
                simulator,
                EVENT_ID_COM_RADIO_SET_HZ,
                EVENT_ID_COM_STBY_RADIO_SET_HZ,
            );
        }
        ModeSelectorState::ModeSelectorCom2 => {
            apply_com_input(
                &mut state.com2_state,
                input.button_lower,
                input.rotary_lower_outer,
                input.rotary_lower_inner,
            );
            display_frequency_on_hardware(
                radio_panel,
                &state.com2_state,
                Window::BottomLeft,
                Window::BottomRight,
                3,
            );
            *connected_to_sim = send_com_to_sim(
                &mut state.com2_state,
                simulator,
                EVENT_ID_COM2_RADIO_SET_HZ,
                EVENT_ID_COM2_STBY_RADIO_SET_HZ,
            );
        }
        ModeSelectorState::ModeSelectorNav1 => {
            apply_nav_input(
                &mut state.nav1_state,
                input.button_lower,
                input.rotary_lower_outer,
                input.rotary_lower_inner,
            );
            display_frequency_on_hardware(
                radio_panel,
                &state.nav1_state,
                Window::BottomLeft,
                Window::BottomRight,
                2,
            );
            *connected_to_sim = send_nav_to_sim(
                &mut state.nav1_state,
                simulator,
                EVENT_ID_NAV1_RADIO_SET_HZ,
                EVENT_ID_NAV1_STBY_SET_HZ,
            );
        }
        ModeSelectorState::ModeSelectorNav2 => {
            apply_nav_input(
                &mut state.nav2_state,
                input.button_lower,
                input.rotary_lower_outer,
                input.rotary_lower_inner,
            );
            display_frequency_on_hardware(
                radio_panel,
                &state.nav2_state,
                Window::BottomLeft,
                Window::BottomRight,
                2,
            );
            *connected_to_sim = send_nav_to_sim(
                &mut state.nav1_state,
                simulator,
                EVENT_ID_NAV2_RADIO_SET_HZ,
                EVENT_ID_NAV2_STBY_SET_HZ,
            );
        }
        ModeSelectorState::ModeSelectorAdf => {
            display_adf_values(
                &state.adf_state,
                Window::BottomLeft,
                Window::BottomRight,
                radio_panel,
            );
        }
        ModeSelectorState::ModeSelectorDme => {
            display_dme_on_hardware(
                radio_panel,
                &state.dme_state,
                &state.nav1_state,
                Window::BottomLeft,
                Window::BottomRight,
            );
        }
        ModeSelectorState::ModeSelectorXpdr => {
            apply_xpdr_input(
                &mut state.xpdr_state,
                input.button_lower,
                input.rotary_lower_outer,
                input.rotary_lower_inner,
            );
            display_xpdr_on_hardware(
                radio_panel,
                &state.xpdr_state,
                Window::BottomLeft,
                Window::BottomRight,
            );
        }
    }
}

fn display_frequency_on_hardware(
    radio_panel: &mut RadioPanel,
    frequency_state: &FrequencyState,
    left_window: Window,
    right_window: Window,
    fractional_digits: u8,
) {
    radio_panel.set_window(
        left_window,
        &format_frequency(frequency_state.active_freq, fractional_digits),
    );
    radio_panel.set_window(
        right_window,
        &format_frequency(frequency_state.standby_freq, fractional_digits),
    );
    radio_panel.update_all_windows();
}

fn display_xpdr_on_hardware(
    radio_panel: &mut RadioPanel,
    state: &XpdrState,
    left_window: Window,
    right_window: Window,
) {
    let code = state.code.map(|d| d.to_string()).join("");
    let code = format!(" {}", code);
    let code: String = code
        .chars()
        .enumerate()
        .map(|(i, char)| {
            if i == (state.selected_digit + 1) {
                format!("{}.", char) // if digit selected, add dot to it
            } else {
                char.to_string()
            }
        })
        .collect();

    radio_panel.set_window(left_window, "     ");
    radio_panel.set_window(right_window, &code);
    radio_panel.update_all_windows();
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

fn display_dme_on_hardware(
    radio_panel: &mut RadioPanel,
    dme_state: &DmeState,
    nav1_state: &FrequencyState,
    window_active: Window,
    window_standby: Window,
) {
    let formatted_distance = format!("   {:.1}", dme_state.distance);
    radio_panel.set_window(window_active, &format_frequency(nav1_state.active_freq, 2));
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

fn apply_com_input(
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
    frequency_state.standby_freq.fraction += match inner_rotary {
        RotaryState::Clockwise => 5,
        RotaryState::CounterClockwise => -5,
        RotaryState::None => 0,
    };

    frequency_state.standby_freq.integer = wrap(frequency_state.standby_freq.integer, 118, 137);
    frequency_state.standby_freq.fraction = wrap(frequency_state.standby_freq.fraction, 0, 1000);
}

fn apply_nav_input(
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
    frequency_state.standby_freq.fraction += match inner_rotary {
        RotaryState::Clockwise => 50,
        RotaryState::CounterClockwise => -50,
        RotaryState::None => 0,
    };

    frequency_state.standby_freq.integer = wrap(frequency_state.standby_freq.integer, 108, 118);
    frequency_state.standby_freq.fraction = wrap(frequency_state.standby_freq.fraction, 0, 1000);
}

fn apply_xpdr_input(
    state: &mut XpdrState,
    swap_button: ButtonState,
    outer_rotary: RotaryState,
    inner_rotary: RotaryState,
) {
    if matches!(swap_button, ButtonState::Pressed) {
        state.selected_digit += 1;
        state.selected_digit = wrap(state.selected_digit.try_into().unwrap(), 0, 4) as usize;
    }
    state.code[state.selected_digit] += match outer_rotary {
        RotaryState::Clockwise => 1,
        RotaryState::CounterClockwise => -1,
        RotaryState::None => 0,
    };
    state.code[state.selected_digit] += match inner_rotary {
        RotaryState::Clockwise => 1,
        RotaryState::CounterClockwise => -1,
        RotaryState::None => 0,
    };
    state.code[state.selected_digit] = wrap(state.code[state.selected_digit], 0, 8);
}

fn apply_autopilot_input(
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

fn send_com_to_sim(
    com_state: &mut FrequencyState,
    simulator: &SimConnector,
    active_event_id: u32,
    standby_event_id: u32,
) -> bool {
    // Format for FS2020
    let active_frequency = parse::<u32>(&format!(
        "{:0>3}{:0>3}000",
        com_state.active_freq.integer, com_state.active_freq.fraction
    ))
    .unwrap();
    let standby_frequency = parse::<u32>(&format!(
        "{:0>3}{:0>3}000",
        com_state.standby_freq.integer, com_state.standby_freq.fraction
    ))
    .unwrap();

    if !simulator.transmit_client_event(1, active_event_id, active_frequency, 5, 0) {
        return false;
    };
    if !simulator.transmit_client_event(1, standby_event_id, standby_frequency, 5, 0) {
        return false;
    }

    true
}

fn send_nav_to_sim(
    nav_state: &mut FrequencyState,
    simulator: &SimConnector,
    active_event_id: u32,
    standby_event_id: u32,
) -> bool {
    return send_com_to_sim(nav_state, simulator, active_event_id, standby_event_id);
}

fn display_adf_values(
    adf_state: &AdfState,
    window_active: Window,
    window_standby: Window,
    radio_panel: &mut RadioPanel,
) {
    radio_panel.set_window(window_active, "     ");
    radio_panel.set_window(
        window_standby,
        &format!("{: >5}", adf_state.standby_frequency),
    );
    radio_panel.update_all_windows();
}

fn swap_frequencies(frequency_state: &mut FrequencyState) {
    std::mem::swap(
        &mut frequency_state.active_freq,
        &mut frequency_state.standby_freq,
    )
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
