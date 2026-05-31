mod gui;
mod macos_gui;
mod windows_gui;
mod render;

use render::render;
use macos_gui::MacFactory;
use windows_gui::WindowsFactory;
use gui::traits::GuiFactoryDynamic;

fn main() {
    let windows = false;

    let mac = MacFactory;
    let win = WindowsFactory;

    // reference to trait object
    let factory: &dyn GuiFactoryDynamic = if windows {
        &win
    } else {
        &mac
    };

    // direct usage
    let button = factory.create_button();
    button.press();

    // pass to function
    render(factory);
}