mod gui;
mod model;

use iced::Theme;
use crate::model::application::OptiRust;

fn main() -> iced::Result {
    iced::application("OptiRust", OptiRust::update, OptiRust::view)
    .theme(|_| {Theme::TokyoNight})
    .run()
}