use iced::{Element};
use iced::widget::canvas;
use crate::model::application::OptiRust;
use crate::gui::chart;
use crate::gui::update::Message;

impl OptiRust {
    pub fn view(&self) -> Element<Message> {
        canvas(&self.chart).width(chart::CHART_WIDTH).height(chart::CHART_HEIGHT).into()
    }
}
