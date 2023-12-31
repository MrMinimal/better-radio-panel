/// The Radio Panel has 4 windows with 5 7-segement displays each
pub const DEVICE_WINDOW_COUNT: usize = 4;

/// Each window has 5 7-segment displays
pub const DEVICE_SEVEN_SEGMENT_COUNT: usize = 5;

/// Show nothing instead of a digit
pub const DIGIT_BLANK: u8 = 0b00001111;

/// Show a dash instead of a digit
pub const DIGIT_DASH: u8 = 0b11100000;

/// Encode that a digit should show a point next to it
pub const BITMASK_SHOW_DECIMAL_POINT: u8 = 0b1101_0000;

/// Encode that a digit shouldn't show a point next to it
pub const BITMASK_HIDE_DECIMAL_POINT: u8 = 0b0000_0000;

/* Which bit has to be 1 to represent button pressed/released */
pub const BITMASK_BUTTON_UPPER_PRESSED: u32 = 0b0000_0000_0000_0000_0100_0000_0000_0000;
pub const BITMASK_BUTTON_LOWER_PRESSED: u32 = 0b0000_0000_0000_0000_1000_0000_0000_0000;

/* Which bit has to be 1 to represent the rotary turning */
pub const BITMASK_ROTARY_UPPER_INNER_CLOCKWISE: u32 = 0b0000_0000_0000_0000_0000_0000_0000_0001;
pub const BITMASK_ROTARY_UPPER_INNER_COUNTERCLOCKWISE: u32 =
    0b0000_0000_0000_0000_0000_0000_0000_0010;
pub const BITMASK_ROTARY_UPPER_OUTER_CLOCKWISE: u32 = 0b0000_0000_0000_0000_0000_0000_0000_0100;
pub const BITMASK_ROTARY_UPPER_OUTER_COUNTERCLOCKWISE: u32 =
    0b0000_0000_0000_0000_0000_0000_0000_1000;
pub const BITMASK_ROTARY_LOWER_INNER_CLOCKWISE: u32 = 0b0000_0000_0000_0000_0000_0000_0001_0000;

/* Which bit has to be 1 to represent the rotary turning */
pub const BITMASK_ROTARY_LOWER_INNER_COUNTERCLOCKWISE: u32 =
    0b0000_0000_0000_0000_0000_0000_0010_0000;
pub const BITMASK_ROTARY_LOWER_OUTER_CLOCKWISE: u32 = 0b0000_0000_0000_0000_0000_0000_0100_0000;
pub const BITMASK_ROTARY_LOWER_OUTER_COUNTERCLOCKWISE: u32 =
    0b0000_0000_0000_0000_0000_0000_1000_0000;

/* Which bit represents what mode on the rotary */
pub const BITMASK_MODE_SELECTOR_UPPER_COM1: u32 = 0b0000_0000_0000_0001_0000_0000_0000_0000;
pub const BITMASK_MODE_SELECTOR_UPPER_COM2: u32 = 0b0000_0000_0000_0010_0000_0000_0000_0000;
pub const BITMASK_MODE_SELECTOR_UPPER_NAV1: u32 = 0b0000_0000_0000_0100_0000_0000_0000_0000;
pub const BITMASK_MODE_SELECTOR_UPPER_NAV2: u32 = 0b0000_0000_0000_1000_0000_0000_0000_0000;
pub const BITMASK_MODE_SELECTOR_UPPER_ADF: u32 = 0b0000_0000_0001_0000_0000_0000_0000_0000;
pub const BITMASK_MODE_SELECTOR_UPPER_DME: u32 = 0b0000_0000_0010_0000_0000_0000_0000_0000;
// pub const BITMASK_MODE_SELECTOR_UPPER_XPDR: u32 = 0b0000_0000_0100_0000_0000_0000_0000_0000;

/* Which bit represents what mode on the rotary */
pub const BITMASK_MODE_SELECTOR_LOWER_COM1: u32 = 0b0000_0000_1000_0000_0000_0000_0000_0000;
pub const BITMASK_MODE_SELECTOR_LOWER_COM2: u32 = 0b0000_0000_0000_0000_0000_0001_0000_0000;
pub const BITMASK_MODE_SELECTOR_LOWER_NAV1: u32 = 0b0000_0000_0000_0000_0000_0010_0000_0000;
pub const BITMASK_MODE_SELECTOR_LOWER_NAV2: u32 = 0b0000_0000_0000_0000_0000_0100_0000_0000;
pub const BITMASK_MODE_SELECTOR_LOWER_ADF: u32 = 0b0000_0000_0000_0000_0000_1000_0000_0000;
pub const BITMASK_MODE_SELECTOR_LOWER_DME: u32 = 0b0000_0000_0000_0000_0001_0000_0000_0000;
// pub const BITMASK_MODE_SELECTOR_LOWER_XPDR: u32 = 0b0000_0000_0000_0000_0010_0000_0000_0000;
