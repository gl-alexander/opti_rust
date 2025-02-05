mod gui;
mod model;

use crate::model::application::OptiRust;


fn main() -> iced::Result {
    iced::application("OptiRust", OptiRust::update, OptiRust::view).run()
}