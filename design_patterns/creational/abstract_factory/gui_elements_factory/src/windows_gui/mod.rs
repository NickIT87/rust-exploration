use crate::gui::button::Button;
use crate::gui::checkbox::Checkbox;
use crate::gui::traits::GuiFactory;

pub struct WindowsButton;
pub struct WindowsCheckbox;

impl Button for WindowsButton {
    fn press(&self) {
        println!("Windows button pressed");
    }
}

impl Checkbox for WindowsCheckbox {
    fn switch(&self) {
        println!("Windows checkbox switched");
    }
}

pub struct WindowsFactory;

impl GuiFactory for WindowsFactory {
    type B = WindowsButton;
    type C = WindowsCheckbox;

    fn create_button(&self) -> Self::B {
        WindowsButton
    }

    fn create_checkbox(&self) -> Self::C {
        WindowsCheckbox
    }
}