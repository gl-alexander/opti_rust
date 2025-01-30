mod gui;

fn main() -> iced::Result {
    iced::run("OptiRust", gui::update::update, gui::view::view)
}
