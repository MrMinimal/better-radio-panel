use core::panic;
use hidapi::{HidApi, HidDevice};
use std::process;

use super::{constants::*, hardware::*};

const VENDOR_ID: u16 = 0x06a3; // Saitek
const PRODUCT_ID: u16 = 0x0d05; // Radio Panel

const CONTROL_MESSAGE_SIZE: usize = 23; // 2 bytes at end unused, required on Windows hidapi

const NO_INPUTS_AFTER_TIMEOUT: u32 = 0;

#[derive(Copy, Clone, Debug)]
pub struct InputState {
    pub mode_selector_upper: ModeSelectorState,
    pub mode_selector_lower: ModeSelectorState,
    pub rotary_upper_inner: RotaryState,
    pub rotary_upper_outer: RotaryState,
    pub rotary_lower_inner: RotaryState,
    pub rotary_lower_outer: RotaryState,
    pub button_upper: ButtonState,
    pub button_lower: ButtonState,
}

impl InputState {
    pub fn new() -> InputState {
        InputState {
            mode_selector_upper: ModeSelectorState::ModeSelectorCom1,
            mode_selector_lower: ModeSelectorState::ModeSelectorCom1,
            rotary_upper_inner: RotaryState::None,
            rotary_upper_outer: RotaryState::None,
            rotary_lower_inner: RotaryState::None,
            rotary_lower_outer: RotaryState::None,
            button_upper: ButtonState::Released,
            button_lower: ButtonState::Released,
        }
    }
}

/// Represents the radio panel with its 4 windows, containing 5 7-segment displays each.
pub struct RadioPanel {
    hid_device: HidDevice,
    windows: [RadioPanelWindow; 4],
}

impl RadioPanel {
    pub fn new() -> RadioPanel {
        RadioPanel {
            hid_device: HidApi::new()
                .unwrap()
                .open(VENDOR_ID, PRODUCT_ID)
                .unwrap_or_else(|_error| {
                    println!("Couldn't connect to hardware. Is it plugged in?");
                    process::exit(1);
                }),
            windows: [RadioPanelWindow {
                displays: [SevenSegmentDisplay {
                    value: DIGIT_BLANK,
                    has_decimal_point: false,
                }; DEVICE_SEVEN_SEGMENT_COUNT], // repeat for all 7-segement displays
            }; DEVICE_WINDOW_COUNT], // repeat for all windows
        }
    }

    /// Blocking call to wait for input from buttons or rotaries
    /// Returns the current state of all buttons and potis on the hardware
    pub fn block_until_input(&mut self) -> Option<InputState> {
        let mut input_buffer = [0u8; 3];
        self.hid_device
            .read_timeout(&mut input_buffer, 300)
            .expect("Error reading from device");

        // Turn buffer array into a single 32 bit value
        let input_buffer = ((input_buffer[0] as u32) << 16)
            | ((input_buffer[1] as u32) << 8)
            | (input_buffer[2] as u32);
        // println!("{:#034b}", buffer)

        if input_buffer == NO_INPUTS_AFTER_TIMEOUT {
            return None;
        }

        let mut input_state = InputState::new();
        input_state.button_upper = parse_button_state(input_buffer, BITMASK_BUTTON_UPPER_PRESSED);
        input_state.button_lower = parse_button_state(input_buffer, BITMASK_BUTTON_LOWER_PRESSED);

        input_state.rotary_upper_inner = parse_rotary_state(
            input_buffer,
            BITMASK_ROTARY_UPPER_INNER_CLOCKWISE,
            BITMASK_ROTARY_UPPER_INNER_COUNTERCLOCKWISE,
        );

        input_state.rotary_upper_outer = parse_rotary_state(
            input_buffer,
            BITMASK_ROTARY_UPPER_OUTER_CLOCKWISE,
            BITMASK_ROTARY_UPPER_OUTER_COUNTERCLOCKWISE,
        );

        input_state.rotary_lower_inner = parse_rotary_state(
            input_buffer,
            BITMASK_ROTARY_LOWER_INNER_CLOCKWISE,
            BITMASK_ROTARY_LOWER_INNER_COUNTERCLOCKWISE,
        );

        input_state.rotary_lower_outer = parse_rotary_state(
            input_buffer,
            BITMASK_ROTARY_LOWER_OUTER_CLOCKWISE,
            BITMASK_ROTARY_LOWER_OUTER_COUNTERCLOCKWISE,
        );

        input_state.mode_selector_upper = parse_mode_selector_state(
            input_buffer,
            ModeSelectorBitmaps {
                com1: BITMASK_MODE_SELECTOR_UPPER_COM1,
                com2: BITMASK_MODE_SELECTOR_UPPER_COM2,
                nav1: BITMASK_MODE_SELECTOR_UPPER_NAV1,
                nav2: BITMASK_MODE_SELECTOR_UPPER_NAV2,
                adf: BITMASK_MODE_SELECTOR_UPPER_ADF,
                dme: BITMASK_MODE_SELECTOR_UPPER_DME,
                xpdr: BITMASK_MODE_SELECTOR_UPPER_XPDR,
            },
        );

        input_state.mode_selector_lower = parse_mode_selector_state(
            input_buffer,
            ModeSelectorBitmaps {
                com1: BITMASK_MODE_SELECTOR_LOWER_COM1,
                com2: BITMASK_MODE_SELECTOR_LOWER_COM2,
                nav1: BITMASK_MODE_SELECTOR_LOWER_NAV1,
                nav2: BITMASK_MODE_SELECTOR_LOWER_NAV2,
                adf: BITMASK_MODE_SELECTOR_LOWER_ADF,
                dme: BITMASK_MODE_SELECTOR_LOWER_DME,
                xpdr: BITMASK_MODE_SELECTOR_LOWER_XPDR,
            },
        );

        Some(input_state)
    }

    /// Show values in window, previous values are cleared
    /// value can contain up to 5 digits from 0-9 and an optional point.
    /// Can also be a blank space or a dash
    /// Examples: 12345, 123.45, 1.2.3.4.5., 12 45, 12-45
    pub fn set_window(&mut self, window: Window, value: &str) {
        self.set_window_additively(window, "     "); // clean window contents
        self.set_window_additively(window, value);
    }

    /// Draws the values into a window without clearing it
    /// If not all digits are set, previous digits can remain
    fn set_window_additively(&mut self, window: Window, value: &str) {
        let window_index = window as usize;
        let mut display_index = 0;

        for character in value.chars() {
            // If a dot succeeds a digit, tell that previous digit it has a decimal point
            if character == '.' {
                self.windows[window_index].displays[display_index - 1].has_decimal_point = true;
                continue;
            }

            self.windows[window_index].displays[display_index].has_decimal_point = false;
            self.windows[window_index].displays[display_index].value = match character {
                ' ' => DIGIT_BLANK,
                '-' => DIGIT_DASH,
                '0'..='9' => character.to_digit(10).unwrap() as u8,
                _ => panic!("Impossible value for 7-segement to display"),
            };

            display_index += 1;
        }
    }

    /// Show the data on all displays
    pub fn update_all_windows(&self) {
        let mut output_buffer = [DIGIT_BLANK; CONTROL_MESSAGE_SIZE];

        // Turn stored data into data buffer to send to device
        // Encoded like this (x meaning irrelevant)
        // 0000xxxx Binary encoded decimal (0x00 shows 0, 0x01 shows 1, etc.)
        // 00001111 Turns the number off
        // 1101xxxx Adds a dot to the number
        // 1110xxxx Shows dash/minus
        for (window_index, window) in self.windows.iter().enumerate() {
            for (display_index, display) in window.displays.iter().enumerate() {
                output_buffer[(5 * window_index) + (display_index + 1)] = display.get_data();
                // I don't know why the display_index has to be offset by 1
            }
        }

        // Send to hardware to display
        output_buffer[0] = 0; // I don't know why this is required
        self.hid_device.send_feature_report(&output_buffer).unwrap();
    }

    pub fn clear_all_windows(&mut self) {
        self.set_window(Window::TopLeft, "     ");
        self.set_window(Window::TopRight, "     ");
        self.set_window(Window::BottomLeft, "     ");
        self.set_window(Window::BottomRight, "     ");
        self.update_all_windows();
    }
}

/// Returns what state is a mode selector is in
fn parse_mode_selector_state(input_buffer: u32, bitmaps: ModeSelectorBitmaps) -> ModeSelectorState {
    if bitmask_applies(input_buffer, bitmaps.com1) {
        ModeSelectorState::ModeSelectorCom1
    } else if bitmask_applies(input_buffer, bitmaps.com2) {
        ModeSelectorState::ModeSelectorCom2
    } else if bitmask_applies(input_buffer, bitmaps.nav1) {
        ModeSelectorState::ModeSelectorNav1
    } else if bitmask_applies(input_buffer, bitmaps.nav2) {
        ModeSelectorState::ModeSelectorNav2
    } else if bitmask_applies(input_buffer, bitmaps.adf) {
        ModeSelectorState::ModeSelectorAdf
    } else if bitmask_applies(input_buffer, bitmaps.dme) {
        ModeSelectorState::ModeSelectorDme
    } else if bitmask_applies(input_buffer, bitmaps.xpdr) {
        ModeSelectorState::ModeSelectorXpdr
    } else {
        panic!(
            "Error in input mask parsing. Unknown bitmask: {:#034b}",
            input_buffer
        )
    }
}

/// Return what state a rotary is in
/// Bitmask defines what rotary is referenced
fn parse_rotary_state(
    input_buffer: u32,
    bitmask_clockwise: u32,
    bitmask_counterclockwise: u32,
) -> RotaryState {
    if bitmask_applies(input_buffer, bitmask_clockwise) {
        RotaryState::Clockwise
    } else if bitmask_applies(input_buffer, bitmask_counterclockwise) {
        RotaryState::CounterClockwise
    } else {
        RotaryState::None
    }
}

/// Return the state a specific button is in
/// Bitmask defines what button is referenced
fn parse_button_state(input_buffer: u32, bitmask_button_pressed: u32) -> ButtonState {
    if bitmask_applies(input_buffer, bitmask_button_pressed) {
        ButtonState::Pressed
    } else {
        ButtonState::Released
    }
}

/// Does a passed bitmask apply?
/// True if all 1 bits in the bitmask are also 1 in the buffer
fn bitmask_applies(input_buffer: u32, bitmask: u32) -> bool {
    (input_buffer & bitmask) > 0
}
