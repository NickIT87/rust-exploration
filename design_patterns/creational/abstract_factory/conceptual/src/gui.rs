pub trait Button {
    fn draw(&self);
}

pub trait Window {
    fn draw(&self);
}

pub trait GuiFactory {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_window(&self) -> Box<dyn Window>;
}