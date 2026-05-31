use crate::gui::{Button, GuiFactory, Window};

pub struct MacButton;

impl Button for MacButton {
    fn draw(&self) {
        println!("Mac Button");
    }
}

pub struct MacWindow;

impl Window for MacWindow {
    fn draw(&self) {
        println!("Mac Window");
    }
}

pub struct MacFactory;

impl GuiFactory for MacFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacButton)
    }

    fn create_window(&self) -> Box<dyn Window> {
        Box::new(MacWindow)
    }
}