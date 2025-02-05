use iced::widget::{button, text};
use iced::{Element};
use iced::widget::canvas::Canvas;
use iced::widget::TextInput;
use iced::widget::keyed::Column;
use iced::widget::column;
use iced::widget::row;
use iced::widget::canvas;
use iced::widget::canvas::Path;
use iced::Length;
use crate::model::application::OptiRust;
use crate::gui::update::Message;
use iced::mouse;
use iced::{Color, Rectangle, Renderer, Theme};


impl OptiRust {
    pub fn view(&self) -> Element<Message> {
        // let chart = Canvas::new(&mut self.chart)
        //     .width(Length::FillPortion(3))
        //     .height(Length::Fill);
        row![
            column![
                canvas(&self.chart),
                // canvas(Circle {radius: 50.0}),
                button("Import"), 
                ].spacing(10),
            button("Run")
        ].spacing(10)
        .into()
        // let input_column = Column::new()
        //     .spacing(10)
        //     .width(Length::FillPortion(2))
        //     .push(
        //         TextInput::new(
        //             "Stock price",
        //             &self.s
        //         )
        //         .padding(10),
        //     )
        //     .push(
        //         TextInput::new(
        //             &mut self.k_input,
        //             "Strike price",
        //             &self.k,
        //             Message::KChanged,
        //         )
        //         .padding(10),
        //     )
        //     .push(
        //         TextInput::new(
        //             &mut self.r_input,
        //             "Risk-free rate",
        //             &self.r,
        //             Message::RChanged,
        //         )
        //         .padding(10),
        //     )
        //     .push(
        //         TextInput::new(
        //             &mut self.sigma_input,
        //             "Volatility",
        //             &self.sigma,
        //             Message::SigmaChanged,
        //         )
        //         .padding(10),
        //     )
        //     .push(
        //         TextInput::new(
        //             &mut self.t_input,
        //             "Time to expiration",
        //             &self.t,
        //             Message::TChanged,
        //         )
        //         .padding(10),
        //     )
        //     .push(
        //         TextInput::new(
        //             &mut self.num_simulations_input,
        //             "Number of simulations",
        //             &self.num_simulations,
        //             Message::NumSimulationsChanged,
        //         )
        //         .padding(10),
        //     )
        //     .push(
        //         TextInput::new(
        //             &mut self.num_steps_input,
        //             "Number of steps",
        //             &self.num_steps,
        //             Message::NumStepsChanged,
        //         )
        //         .padding(10),
        //     )
        //     .push(
        //         Button::new(&mut self.run_button, Text::new("Run Simulation"))
        //             .on_press(Message::RunSimulation)
        //             .padding(10),
        //     )
        //     .push(
        //         Button::new(
        //             &mut button::State::new(),
        //             Text::new(if self.is_call { "Call Option" } else { "Put Option" }),
        //         )
        //         .on_press(Message::ToggleOptionType)
        //         .padding(10),
        //     );
    
        // let result_text = if let Some(result) = self.result {
        //     format!("Option Price: {:.4}", result)
        // } else {
        //     "Run simulation to see results".to_string()
        // };
    
        // let content = Row::new()
        //     .spacing(20)
        //     .padding(20)
        //     .push(chart)
        //     .push(
        //         Column::new()
        //             .width(Length::FillPortion(2))
        //             .spacing(20)
        //             .push(input_column)
        //             .push(Text::new(result_text).size(20)),
        //     );
    
        // Element::new(content)
    }
}
