use chrono::{DateTime, Local, Timelike};
use iced::{
    mouse, time,
    widget::{
        canvas::{self, Geometry, Path, Stroke},
        Canvas,
    },
    Color, Element, Fill, Pixels, Point, Rectangle, Renderer, Subscription, Task, Theme,
};

fn main() -> iced::Result {
    iced::application("Clock - Iced", Clock::update, Clock::view)
        .subscription(Clock::subscription)
        .antialiasing(true)
        .theme(|_| Theme::Dark)
        .run_with(Clock::new)
}

struct Clock {
    time: DateTime<Local>,
}

struct ClockFace {
    time: DateTime<Local>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(DateTime<Local>),
}

impl Clock {
    fn new() -> (Self, Task<Message>) {
        (Self { time: Local::now() }, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick(local_time) => self.time = local_time,
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        Canvas::new(ClockFace { time: self.time })
            .width(Fill)
            .height(Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(std::time::Duration::from_millis(500)).map(|_| Message::Tick(Local::now()))
    }
}

impl<Message> canvas::Program<Message> for ClockFace {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut clock = canvas::Frame::new(renderer, bounds.size());

        let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
        let radius = bounds.width.min(bounds.height) / 2.0;

        let background = Path::circle(center, radius);
        clock.fill(&background, Color::from_rgb8(255, 255, 255));

        for hour in 1..=12 {
            let angle = (hour as f32 * 30.0).to_radians();
            let start = Point::new(
                center.x + (radius - 20.0) * angle.sin(),
                center.y - (radius - 20.0) * angle.cos(),
            );
            let end = Point::new(
                center.x + (radius - 10.0) * angle.sin(),
                center.y - (radius - 10.0) * angle.cos(),
            );
            let line = Path::line(start, end);
            clock.stroke(&line, Stroke::default().with_width(2.0));

            // Draw hour numbers
            let text = format!("{}", hour);
            let text_position = Point::new(
                center.x + (radius - 40.0) * angle.sin() - 5.0,
                center.y - (radius - 40.0) * angle.cos() + 5.0,
            );
            clock.fill_text(canvas::Text {
                content: text,
                position: text_position,
                color: Color::BLACK,
                size: Pixels { 0: 20.0 },
                ..canvas::Text::default()
            });
        }

        for minute in 0..60 {
            if minute % 5 != 0 {
                let angle = (minute as f32 * 6.0).to_radians();
                let start = Point::new(
                    center.x + (radius - 15.0) * angle.sin(),
                    center.y - (radius - 15.0) * angle.cos(),
                );
                let end = Point::new(
                    center.x + (radius - 10.0) * angle.sin(),
                    center.y - (radius - 10.0) * angle.cos(),
                );
                let line = Path::line(start, end);
                clock.stroke(&line, Stroke::default().with_width(1.0));
            }
        }

        let hour_angle =
            (self.time.hour() as f32 * 30.0 + self.time.minute() as f32 * 0.5).to_radians();
        let hour_hand = Path::line(
            center,
            Point::new(
                center.x + (radius * 0.5) * hour_angle.sin(),
                center.y - (radius * 0.5) * hour_angle.cos(),
            ),
        );
        clock.stroke(&hour_hand, Stroke::default().with_width(4.0));

        let minute_angle = (self.time.minute() as f32 * 6.0).to_radians();
        let minute_hand = Path::line(
            center,
            Point::new(
                center.x + (radius * 0.7) * minute_angle.sin(),
                center.y - (radius * 0.7) * minute_angle.cos(),
            ),
        );
        clock.stroke(&minute_hand, Stroke::default().with_width(3.0));

        let second_angle = (self.time.second() as f32 * 6.0).to_radians();
        let second_hand = Path::line(
            center,
            Point::new(
                center.x + (radius * 0.9) * second_angle.sin(),
                center.y - (radius * 0.9) * second_angle.cos(),
            ),
        );
        clock.stroke(
            &second_hand,
            Stroke::default().with_width(1.0).with_color(Color::BLACK),
        );

        vec![clock.into_geometry()]
    }
}
