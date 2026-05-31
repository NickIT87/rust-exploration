use crate::gui::button::Button;
use crate::gui::checkbox::Checkbox;
use crate::gui::traits::GuiFactoryDynamic;

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

impl GuiFactoryDynamic for MacFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(MacCheckbox)
    }
}