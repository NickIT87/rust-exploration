use crate::gui::button::Button;
use crate::gui::checkbox::Checkbox;
use crate::gui::traits::GuiFactoryDynamic;

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

impl GuiFactoryDynamic for WindowsFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(WindowsCheckbox)
    }
}