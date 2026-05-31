mod gui;
mod macos_gui;
mod windows_gui;
mod render;

use render::render;
use macos_gui::MacFactory;
use windows_gui::WindowsFactory;

fn main() {
    let windows = false;

    if windows {
        render(WindowsFactory);
    } else {
        render(MacFactory);
    }
}