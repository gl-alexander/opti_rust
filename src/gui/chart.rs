use core::num;

use crate::model::chart::{self, PriceChart};
use crate::gui::update::Message;
use iced::{mouse, Color, Point, Rectangle, Renderer, Theme, Size, Pixels};
use iced::widget::canvas;
use iced::widget::canvas::{Frame, Path, Stroke, Text};

const BOUNDS_OFFSET: f32 = 50.0;
const INNER_OFFSET: f32 = 15.0;
const MAX_POINT_SHOWN: usize = 30;
const POINT_RADIUS: f32 = 5.0;
const VERTICAL_SCALING: f32 = 0.7; // represents what % of the canvas we should put the highest price of the PriceChart
// 239, 98, 108
const COLOR_MAX_VAL: f32 = 255f32;
const COLOR_WHITE: Color = Color{r: 246f32 / COLOR_MAX_VAL, g: 232f32 / COLOR_MAX_VAL, b: 234f32 / COLOR_MAX_VAL, a:1f32};
const COLOR_BLUE: Color = Color{r: 132f32 / COLOR_MAX_VAL, g: 220f32 / COLOR_MAX_VAL, b: 207f32 / COLOR_MAX_VAL, a:1f32};
const COLOR_RED: Color = Color{r: 239f32 / COLOR_MAX_VAL, g: 98f32 / COLOR_MAX_VAL, b: 108f32 / COLOR_MAX_VAL, a:1f32};


impl canvas::Program<Message> for PriceChart {
    type State = PriceChart;

    fn draw(
            &self,
            state: &Self::State,
            renderer: &Renderer,
            theme: &Theme,
            bounds: iced::Rectangle,
            cursor: mouse::Cursor,
        ) -> Vec<canvas::Geometry<Renderer>> {
            let mut frame = Frame::new(renderer, bounds.size());
        
        let drawing_bounds = Rectangle::new(
            Point::new(INNER_OFFSET, INNER_OFFSET), 
            Size::new(bounds.width - INNER_OFFSET - BOUNDS_OFFSET, bounds.height - INNER_OFFSET - BOUNDS_OFFSET)
        );

        // Draw axes
        let x_axis = Path::line(Point::new(BOUNDS_OFFSET, bounds.height - BOUNDS_OFFSET), Point::new(bounds.width - BOUNDS_OFFSET, bounds.height - BOUNDS_OFFSET));
        let y_axis = Path::line(Point::new(BOUNDS_OFFSET, BOUNDS_OFFSET), Point::new(BOUNDS_OFFSET, bounds.height - BOUNDS_OFFSET));
        frame.stroke(&x_axis, Stroke::default().with_color(COLOR_WHITE));
        frame.stroke(&y_axis, Stroke::default().with_color(COLOR_WHITE));

        // Draw data points and lines
        let num_points = self.data.len().min(MAX_POINT_SHOWN);
        let min_price = self.min_price as f32;
        let max_price = self.max_price as f32;
        let min_viewing_price = min_price * VERTICAL_SCALING;
        let max_viewing_price = max_price / VERTICAL_SCALING;

        let mut points = vec![];
        for i in 0..num_points {
            let x = INNER_OFFSET + (i as f32 / num_points as f32) * (drawing_bounds.width - BOUNDS_OFFSET);
            let y = (self.data[i].price as f32 - min_viewing_price) / (max_viewing_price - min_viewing_price) * (drawing_bounds.height - BOUNDS_OFFSET);
            points.push(Point::new(x, y));

            let circle = canvas::Path::circle(Point::new(x, y), POINT_RADIUS);
            frame.fill(&circle, COLOR_RED);
        }

        // Draw line connecting points
        let line = Path::new(|p| {
            p.move_to(points[0]);
            for point in points.iter().skip(1) {
                p.line_to(*point);
            }
        });
        frame.stroke(&line, Stroke::default().with_color(COLOR_BLUE));

        // Draw X-axis labels (dates)
        let num_x_labels = 5;
        let first_point_index = self.data.len() - num_points;
        let date_range = self.data.last().unwrap().date  - self.data[first_point_index].date;
        let days_range = date_range.num_days();
        for i in 0..=num_x_labels {
            let date = self.data[first_point_index].date + chrono::Duration::days((i as f32 / num_x_labels as f32 * days_range as f32) as i64);
            let x = drawing_bounds.x + (i as f32 / num_x_labels as f32) * (drawing_bounds.width - BOUNDS_OFFSET);
            let label = Text {
                content: date.format("%Y-%m-%d").to_string(),
                position: Point::new(x, drawing_bounds.y + drawing_bounds.height + BOUNDS_OFFSET / 2.0f32),
                color: Color::BLACK,
                size: Pixels(16.0),
                ..Text::default()
            };
            frame.fill_text(label);
        }

        // Draw Y-axis labels (prices)
        let num_y_labels = 5;
        for i in 0..=num_y_labels {
            let price = min_viewing_price + (i as f32 / num_y_labels as f32) * (max_viewing_price - min_viewing_price);
            let y = INNER_OFFSET + drawing_bounds.height - (i as f32 / num_y_labels as f32) * drawing_bounds.height;
            let label = Text {
                content: format!("{:.2}", price),
                position: Point::new(bounds.width - 50.0, y),
                color: Color::BLACK,
                size: Pixels(16.0),
                ..Text::default()
            };
            frame.fill_text(label);
        }

        vec![frame.into_geometry()]
    }
}