#[derive(Debug)]
pub enum ButtonState {
    Pressed,
    Released,
}

#[derive(Debug)]
pub enum RotaryState {
    None,
    Clockwise,
    CounterClockwise,
}

#[derive(Debug)]
pub enum ModeSelectorState {
    ModeSelectorCom1,
    ModeSelectorCom2,
    ModeSelectorNav1,
    ModeSelectorNav2,
    ModeSelectorAdf,
    ModeSelectorDme,
    ModeSelectorXpdr,
}
