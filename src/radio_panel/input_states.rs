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

#[derive(Copy, Clone, Debug)]
pub enum ModeSelectorState {
    ModeSelectorCom1,
    ModeSelectorCom2,
    ModeSelectorNav1,
    ModeSelectorNav2,
    ModeSelectorAdf,
    ModeSelectorDme,
    ModeSelectorXpdr,
}
