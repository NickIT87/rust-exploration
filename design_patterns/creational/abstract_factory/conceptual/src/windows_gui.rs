use crate::gui::{Button, GuiFactory, Window};

pub struct WindowsButton;

impl Button for WindowsButton {
    fn draw(&self) {
        println!("Windows Button");
    }
}

pub struct WindowsWindow;

impl Window for WindowsWindow {
    fn draw(&self) {
        println!("Windows Window");
    }
}

pub struct WindowsFactory;

impl GuiFactory for WindowsFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton)
    }

    fn create_window(&self) -> Box<dyn Window> {
        Box::new(WindowsWindow)
    }
}