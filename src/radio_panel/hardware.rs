use super::constants::{
    BITMASK_HIDE_DECIMAL_POINT, BITMASK_SHOW_DECIMAL_POINT, DEVICE_SEVEN_SEGMENT_COUNT,
};

#[derive(Copy, Clone, Debug)]
pub enum ButtonState {
    Pressed,
    Released,
}

#[derive(Copy, Clone, Debug)]
pub enum RotaryState {
    None,
    Clockwise,
    CounterClockwise,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ModeSelectorState {
    ModeSelectorCom1,
    ModeSelectorCom2,
    ModeSelectorNav1,
    ModeSelectorNav2,
    ModeSelectorAdf,
    ModeSelectorDme,
    ModeSelectorXpdr,
}

pub struct ModeSelectorBitmaps {
    pub com1: u32,
    pub com2: u32,
    pub nav1: u32,
    pub nav2: u32,
    pub adf: u32,
    pub dme: u32,
    pub xpdr: u32,
}

#[derive(Copy, Clone)]
pub enum Window {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Copy, Clone, Debug)]
/// One of 4 windows on the Radio Panel
/// Each window has 5, 7-segement displays
pub struct RadioPanelWindow {
    pub displays: [SevenSegmentDisplay; DEVICE_SEVEN_SEGMENT_COUNT],
}

#[derive(Copy, Clone, Debug)]
pub struct SevenSegmentDisplay {
    pub value: u8,               // can only display from 0 to 9 or a dash
    pub has_decimal_point: bool, // used to show a decimal point right of number
}

impl SevenSegmentDisplay {
    /// Returns both value and decimal point data combined
    pub fn get_data(&self) -> u8 {
        if self.has_decimal_point {
            self.value | BITMASK_SHOW_DECIMAL_POINT // set magic first 4 bits to show a decimal point
        } else {
            self.value | BITMASK_HIDE_DECIMAL_POINT // don't set magic first 4 bits
        }
    }
}
