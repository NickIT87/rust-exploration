use crate::gui::button::Button;
use crate::gui::checkbox::Checkbox;
use crate::gui::traits::GuiFactory;

pub struct MacButton;
pub struct MacCheckbox;

impl Button for MacButton {
    fn press(&self) {
        println!("Mac button pressed");
    }
}

impl Checkbox for MacCheckbox {
    fn switch(&self) {
        println!("Mac checkbox switched");
    }
}

pub struct MacFactory;

impl GuiFactory for MacFactory {
    type B = MacButton;
    type C = MacCheckbox;

    fn create_button(&self) -> Self::B {
        MacButton
    }

    fn create_checkbox(&self) -> Self::C {
        MacCheckbox
    }
}