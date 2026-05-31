use crate::gui::traits::GuiFactoryDynamic;

pub fn render(factory: &dyn GuiFactoryDynamic) {
    let button = factory.create_button();
    let checkbox = factory.create_checkbox();
    checkbox.switch();
    button.press();
}