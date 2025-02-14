use crate::model::chart::PriceChart;
use crate::gui::update::Message;
use iced::{mouse, Color, Point, Rectangle, Renderer, Theme, Size, Pixels, event::Status};
use iced::widget::canvas;
use iced::widget::canvas::{Frame, Path, Stroke, Text, Event};

pub const CHART_WIDTH: f32 = 1000f32;
pub const CHART_HEIGHT: f32 = 600f32;

const BOUNDS_OFFSET: f32 = 50.0;
const INNER_OFFSET: f32 = 15.0;
const POINT_RADIUS: f32 = 5.0;
const VERTICAL_SCALING: f32 = 1f32 / 8f32; // represents what % of the canvas we should put the highest price of the PriceChart
// 239, 98, 108
const COLOR_MAX_VAL: f32 = 255f32;
const COLOR_WHITE: Color = Color{r: 246f32 / COLOR_MAX_VAL, g: 232f32 / COLOR_MAX_VAL, b: 234f32 / COLOR_MAX_VAL, a:1f32};
const COLOR_BLUE: Color = Color{r: 132f32 / COLOR_MAX_VAL, g: 220f32 / COLOR_MAX_VAL, b: 207f32 / COLOR_MAX_VAL, a:1f32};
const COLOR_RED: Color = Color{r: 239f32 / COLOR_MAX_VAL, g: 98f32 / COLOR_MAX_VAL, b: 108f32 / COLOR_MAX_VAL, a:1f32};

pub struct ChartDisplayState {
    pub points: Vec<Point>,
    pub initialized: bool,
    pub min_viewing_price: f32,
    pub max_viewing_price: f32,
    pub price_diff: f32,
    pub hover_index: Option<usize>,
    pub select_index: Option<usize>,
    pub x_labels: Vec<Text>,
    pub y_labels: Vec<Text>,
}

impl Default for ChartDisplayState {
    fn default() -> Self {
        let mut result = ChartDisplayState{
            points: vec![Point::new(0f32, 0f32), Point::new(1f32, 1f32)], 
            initialized: false, 
            min_viewing_price: 0f32, 
            max_viewing_price: 1f32, 
            price_diff: 1f32, 
            hover_index: None, 
            select_index: None, 
            x_labels: vec![Text::default(), Text::default()],
            y_labels: vec![Text::default(), Text::default()],
        };
        result.update_display_points(&PriceChart::default(), Rectangle::new(Point::new(0f32, 0f32), Size::new(CHART_WIDTH, CHART_HEIGHT)));
        result
    }
}

impl canvas::Program<Message> for PriceChart {
    type State = ChartDisplayState;

    fn draw(
            &self,
            state: &Self::State,
            renderer: &Renderer,
            _theme: &Theme,
            bounds: iced::Rectangle,
            cursor: mouse::Cursor,
        ) -> Vec<canvas::Geometry<Renderer>> {
        let mut frame = Frame::new(renderer, bounds.size());
        if !state.initialized {
            return vec![frame.into_geometry()];
        }

        let drawing_bounds = Rectangle::new(
            Point::new(INNER_OFFSET, INNER_OFFSET), 
            Size::new(bounds.width - INNER_OFFSET - BOUNDS_OFFSET, bounds.height - INNER_OFFSET - BOUNDS_OFFSET)
        );

        // Draw axes
        let x_axis = Path::line(Point::new(BOUNDS_OFFSET, bounds.height - BOUNDS_OFFSET), Point::new(bounds.width - BOUNDS_OFFSET, bounds.height - BOUNDS_OFFSET));
        let y_axis = Path::line(Point::new(bounds.width - BOUNDS_OFFSET, BOUNDS_OFFSET), Point::new(bounds.width - BOUNDS_OFFSET, bounds.height - BOUNDS_OFFSET));
        frame.stroke(&x_axis, Stroke::default().with_color(COLOR_WHITE));
        frame.stroke(&y_axis, Stroke::default().with_color(COLOR_WHITE));

        // Draw line connecting points
        let line = Path::new(|p| {
            p.move_to(state.points[0]);
            for point in state.points.iter().skip(1) {
                p.line_to(*point);
            }
        });
        frame.stroke(&line, Stroke::default().with_color(COLOR_BLUE));
        // Draw points
        for p in &state.points {
            let circle = canvas::Path::circle(Point::new(p.x, p.y), POINT_RADIUS);
            frame.fill(&circle, COLOR_RED);
        }

        for label in &state.x_labels {
            frame.fill_text(label.clone());
        }
        
        for label in &state.y_labels {
            frame.fill_text(label.clone());
        }

        if let Some(i) = &state.hover_index {
            let data_point = self.data[*i];
            let point = state.points[*i];
            let rect = canvas::Path::new(|p: &mut canvas::path::Builder| {
                p.move_to(Point::new(point.x - 50.0, point.y - 20.0)); // Position top-left of rect
                p.line_to(Point::new(point.x + 50.0, point.y - 20.0)); // Position top-right
                p.line_to(Point::new(point.x + 50.0, point.y + 20.0)); // Position bottom-right
                p.line_to(Point::new(point.x - 50.0, point.y + 20.0)); // Position bottom-left
                p.close();
            });
    
            frame.fill(&rect, Color::from_rgba(0.0, 0.0, 0.0, 0.5)); // Translucent black
    
            // Draw the price and date inside the rectangle
            let label = Text {
                content: format!("Price: {:.2}\nDate: {}", data_point.price, data_point.date.format("%Y-%m-%d")),
                position: Point::new(point.x - 45.0, point.y - 15.0),
                color: Color::WHITE,
                size: Pixels(16.0),
                ..Text::default()
            };
            frame.fill_text(label);
        }
    

        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: Event,
        bounds: iced::Rectangle,
        cursor: mouse::Cursor,
    ) -> (Status, Option<Message>) {

        if !state.initialized || self.refresh_chart {
            state.update_display_points(self, bounds);
            state.initialized = true;
        }
        match event {
            // Detect mouse movement (for hover effect)
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                if let Some(pos) = cursor.position_in(bounds) {
                    // Find closest point within a small hover radius
                    let hover_radius = 10.0; // Adjust for sensitivity
                    state.hover_index = state.points.iter()
                        .enumerate()
                        .filter(|(_, point)| {
                            let dx = (point.x - pos.x).abs();
                            let dy = (point.y - pos.y).abs();
                            (dx.powi(2) + dy.powi(2)).sqrt() < hover_radius
                        })
                        .map(|(index, _)| index)
                        .next(); // Get the first close point (if any)
                } else {
                    state.hover_index = None; // Reset hover if cursor is out of bounds
                }
            }
            _ => {}
        }
    
        (Status::Captured, None)
    }
}

impl ChartDisplayState {
    fn update_display_points(&mut self, price_chart: &PriceChart, bounds: Rectangle) {
        let drawing_bounds = Rectangle::new(
            Point::new(INNER_OFFSET, INNER_OFFSET),
            Size::new(bounds.width - INNER_OFFSET - BOUNDS_OFFSET, bounds.height - INNER_OFFSET - BOUNDS_OFFSET),
        );
    
        let min_price = price_chart.min_price as f32;
        let max_price = price_chart.max_price as f32;
        let diff = max_price - min_price;
        let scaled_diff = diff * VERTICAL_SCALING;
        self.min_viewing_price = min_price - scaled_diff;
        self.max_viewing_price = max_price + scaled_diff;
        self.price_diff = self.max_viewing_price - self.min_viewing_price;
    
        self.points.clear();
        for (i, data_point) in price_chart.data.iter().enumerate() {
            let x = INNER_OFFSET + (i as f32 / (price_chart.data.len() - 1) as f32) * (drawing_bounds.width - BOUNDS_OFFSET);
            let y = (self.max_viewing_price - data_point.price as f32)
                    / self.price_diff * (drawing_bounds.height - BOUNDS_OFFSET);
            self.points.push(Point::new(x, y));
        }

        // Draw X-axis labels (dates)
        let num_x_labels = 5;
        let date_range = price_chart.data.last().unwrap().date  - price_chart.data[0].date;
        let days_range = date_range.num_days();
        for i in 0..=num_x_labels {
            let date = price_chart.data[0].date + chrono::Duration::days((i as f32 / num_x_labels as f32 * days_range as f32) as i64);
            let x = drawing_bounds.x + (i as f32 / num_x_labels as f32) * (drawing_bounds.width - BOUNDS_OFFSET);
            let label = Text {
                content: date.format("%Y-%m-%d").to_string(),
                position: Point::new(x, drawing_bounds.y + drawing_bounds.height + BOUNDS_OFFSET / 2.0f32),
                color: COLOR_WHITE,
                size: Pixels(16.0),
                ..Text::default()
            };
            self.x_labels.push(label);
        }

        // Draw Y-axis labels (prices)
        let num_y_labels = 5;
        for i in 0..=num_y_labels {
            let price = self.min_viewing_price + (i as f32 / num_y_labels as f32) * self.price_diff;
            let y = INNER_OFFSET + drawing_bounds.height - (i as f32 / num_y_labels as f32) * drawing_bounds.height;
            let label = Text {
                content: format!("{:.2}", price),
                position: Point::new(bounds.width - INNER_OFFSET, y),
                color: COLOR_WHITE,
                size: Pixels(16.0),
                ..Text::default()
            };
            self.y_labels.push(label);
        }
    }
}