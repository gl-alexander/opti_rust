use iced::widget::{button, canvas, center, column, container, mouse_area, opaque, rich_text, row, span, stack, text, text_input, Container, Row};
use iced::{color, font, Color, Element, Font};
use crate::model::application::OptiRust;
use crate::gui::chart;
use crate::gui::update::Message;

const API_KEY_INPUT_WIDTH:u16 = 300;
const STOCK_INPUT_WIDTH: u16 = 150;
const PARAM_WIDTH: u16 = 70;
const PARAM_DESCRIPTION_WIDTH: u16 = 170;

impl OptiRust {
    pub fn view(&self) -> Element<Message> {
        let main_content = column![
            row![
                button("Import").on_press(Message::ShowImport),
                text!("Current Index: {}", self.imported_index),
            ].spacing(10),
            canvas(&self.chart).width(chart::CHART_WIDTH).height(chart::CHART_HEIGHT),
            self.display_monte_carlo_params()

        ].spacing(20);
        if let Some(value) = &self.error_message {
            modal(main_content, text!["Error occured: {}", value], Message::ClearError)
        } else if self.require_api_key {
            modal(main_content, self.display_api_key_input(), Message::HideSubmitApiKey)
        } else if self.show_import {
            modal(main_content, self.display_import_input(), Message::HideImport)
        } else {
            main_content.into()
        }
    }

    fn display_monte_carlo_params(&self) -> Row<'_, Message> {
        let mut mc_result_text = String::from("");
        let mut mc_output = String::from("");
        if let Some(value) = self.pricing_result {
            mc_result_text = String::from("Results from Monte Carlo pricing: ");
            mc_output = value.to_string(); 
        }
        row![
            column![
                row![text!["Asset price: "].width(PARAM_DESCRIPTION_WIDTH), text_input(&self.monte_carlo_params.current_asset_price, &self.monte_carlo_params.current_asset_price).width(PARAM_WIDTH).on_input(Message::AssetPriceChanged)],
                row![text!["Strike price: "].width(PARAM_DESCRIPTION_WIDTH), text_input(&self.monte_carlo_params.strike_price, &self.monte_carlo_params.strike_price).width(PARAM_WIDTH).on_input(Message::StrikePriceChanged)],
                row![text!["Market option price: "].width(PARAM_DESCRIPTION_WIDTH), text_input(&self.monte_carlo_params.market_option_price, &self.monte_carlo_params.market_option_price).width(PARAM_WIDTH).on_input(Message::MarketPriceChanged)],
                row![text!["Implied volatility: "].width(PARAM_DESCRIPTION_WIDTH), text_input(&self.monte_carlo_params.implied_vol, &self.monte_carlo_params.implied_vol).width(PARAM_WIDTH).on_input(Message::ImpliedVolChanged)],
                row![text!["Risk free rate: "].width(PARAM_DESCRIPTION_WIDTH), text_input(&self.monte_carlo_params.risk_free_rate, &self.monte_carlo_params.risk_free_rate).width(PARAM_WIDTH).on_input(Message::RiskFreeRateChanged)],
                row![text!["Days to expire: "].width(PARAM_DESCRIPTION_WIDTH), text_input(&self.monte_carlo_params.days_to_expire, &self.monte_carlo_params.days_to_expire).width(PARAM_WIDTH).on_input(Message::DaysToExpireChanged)],

                row![text!["Number of simulations: "].width(PARAM_DESCRIPTION_WIDTH), text_input(&self.monte_carlo_params.num_simulations, &self.monte_carlo_params.num_simulations).width(PARAM_WIDTH).on_input(Message::NumSimulationsChanged)],
                row![text!["Number of steps: "].width(PARAM_DESCRIPTION_WIDTH), text_input(&self.monte_carlo_params.num_steps, &self.monte_carlo_params.num_steps).width(PARAM_WIDTH).on_input(Message::NumStepsChanged)],
            ].spacing(5),
            column![
                button("Run Monte Carlo").on_press(Message::RunMonteCarlo),
                button("Calculate implied volatility").on_press(Message::UpdateParameters)
            ].spacing(10),
            rich_text![
                span(mc_result_text).color(color!(0xff0000)),
                " ",
                span(mc_output).font(Font { weight: font::Weight::Bold, ..Font::default() }),
            ].size(20)
        ].spacing(20)
    }

    fn display_api_key_input(&self) -> Container<'_, Message> {
        container(
            row![
                text_input("API key", &self.api_key).width(API_KEY_INPUT_WIDTH).on_input(Message::ApiKeyChanged), 
                button("Submit").on_press(Message::SubmitApiKey)
            ]
        )
    }

    fn display_import_input(&self) -> Container<'_, Message> {
        container(
            row![
                text_input("Index name", &self.index_value_text).width(STOCK_INPUT_WIDTH).on_input(Message::ImportIndexChanged), 
                button("Import").on_press(Message::ImportData)
            ]
        )
    }
}

fn modal<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    stack![
        base.into(),
        opaque(
            mouse_area(center(opaque(content)).style(|_theme| {
                container::Style {
                    background: Some(
                        Color {
                            a: 0.8,
                            ..Color::BLACK
                        }
                        .into(),
                    ),
                    ..container::Style::default()
                }
            }))
            .on_press(on_blur)
        )
    ]
    .into()
}
