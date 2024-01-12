use druid::{
    widget::{Button, Flex, Label},
    AppLauncher,
    Data,
    Env, // Widget,
    WindowDesc,
};

use tauri::Manager;
#[derive(Clone, Data)]
struct FunnyData {
    num: i32,
}

fn ui_builder() -> impl druid::Widget<FunnyData> {
    let label = Label::new(|data: &FunnyData, _: &Env| format!("Counter: {}", data.num));
    let increment = Button::new("+").on_click(|_ctx, data: &mut FunnyData, _env| {
        data.num += 1;
    });
    let decrement = Button::new("-").on_click(|_ctx, data: &mut FunnyData, _env| {
        data.num -= 1;
    });

    Flex::column()
        .with_child(label)
        .with_child(Flex::row().with_child(increment).with_child(decrement))
    //druid::TextBox::new()
}
fn main() {
    //Window Descriptor
    // launch to stars

    let main_window = WindowDesc::new(ui_builder()).title("Funny Counter");
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(FunnyData { num: 0 })
        .unwrap();

    // ui using tauri
}
