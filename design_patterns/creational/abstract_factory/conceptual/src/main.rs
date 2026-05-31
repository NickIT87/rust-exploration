mod gui;
mod macos_gui;
mod windows_gui;

use gui::GuiFactory;
use macos_gui::MacFactory;
use windows_gui::WindowsFactory;

fn build_ui(factory: &dyn GuiFactory) {
    let button = factory.create_button();
    let window = factory.create_window();

    button.draw();
    window.draw();
}

fn main() {
    let use_windows = false;

    let factory: Box<dyn GuiFactory> = if use_windows {
        Box::new(WindowsFactory)
    } else {
        Box::new(MacFactory)
    };

    build_ui(factory.as_ref());
}