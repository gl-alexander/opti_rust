use iced::widget::{button, canvas, column, container, row, text_input, stack, opaque, mouse_area, center, text};
use iced::{Bottom, Color, Element, Fill, Subscription, Task};
use crate::model::application::OptiRust;
use crate::gui::chart;
use crate::gui::update::Message;

impl OptiRust {
    pub fn view(&self) -> Element<Message> {
        let main_content = column![
            row![
                button("Import").on_press(Message::ShowImport),
                text!("Current Index: "),
            ].spacing(10),
            canvas(&self.chart).width(chart::CHART_WIDTH).height(chart::CHART_HEIGHT),
            row![
                button("Run Monte Carlo")
            ]
        ].spacing(20);
        if self.require_api_key {
            let api_key_submit = container(
                row![
                    text_input("API key", &self.api_key).on_input(Message::ApiKeyChanged), 
                    button("Submit").on_press(Message::SubmitApiKey)
                ]
            );

            modal(main_content, api_key_submit, Message::HideSubmitApiKey)
        } else if self.show_import {
            let import = container(
                row![
                    text_input("Index name", &self.imported_index).on_input(Message::ImportIndexChanged), 
                    button("Import").on_press(Message::ImportData)
                ]
            );

            modal(main_content, import, Message::HideImport)
            
        } else {
            main_content.into()
        }
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
