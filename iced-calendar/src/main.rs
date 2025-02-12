use std::fmt;

use chrono::{Datelike, Duration, Local, Months, NaiveDateTime};
use iced::{
    widget::{button, column, horizontal_rule, horizontal_space, pick_list, row, text, Column},
    Center, Element, Fill, Task, Theme,
};

fn main() -> iced::Result {
    iced::application("Calendar - Iced", Calendar::update, Calendar::view)
        .theme(|_| Theme::Dark)
        .run_with(Calendar::new)
}

#[allow(dead_code)]
struct Calendar {
    current_date: NaiveDateTime,
    prev_month: button::Status,
    next_month: button::Status,
    today: button::Status,
    view_mode: ViewMode,
    view_mode_picklist: Option<ViewMode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ViewMode {
    Month,
    Week,
    Day,
    List,
}

#[derive(Debug, Clone)]
enum Message {
    PreviousMonth,
    NextMonth,
    Today,
    ChangeView(ViewMode),
}

impl Calendar {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                current_date: Local::now().naive_local(),
                prev_month: button::Status::Active,
                next_month: button::Status::Active,
                today: button::Status::Active,
                view_mode: ViewMode::Month,
                view_mode_picklist: Some(ViewMode::Month),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PreviousMonth => {
                self.current_date = self
                    .current_date
                    .with_day(1)
                    .unwrap()
                    .checked_sub_months(Months::new(1))
                    .unwrap()
                    .with_month0((self.current_date.month0() + 11) % 12)
                    .unwrap_or(self.current_date);
            }
            Message::NextMonth => {
                self.current_date = self
                    .current_date
                    .with_day(1)
                    .unwrap()
                    .checked_add_months(Months::new(1))
                    .unwrap()
                    .with_month0((self.current_date.month0() + 1) % 12)
                    .unwrap_or(self.current_date);
            }
            Message::Today => {
                self.current_date = Local::now().naive_local();
            }
            Message::ChangeView(view) => {
                self.view_mode = view;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let header = row![
            button("Previous").on_press(Message::PreviousMonth),
            button("Today").on_press(Message::Today),
            button("Next").on_press(Message::NextMonth),
            horizontal_space(),
            pick_list(
                &[
                    ViewMode::Month,
                    ViewMode::Week,
                    ViewMode::Day,
                    ViewMode::List,
                ][..],
                Some(self.view_mode),
                Message::ChangeView,
            )
        ]
        .spacing(10)
        .padding(5)
        .align_y(Center);

        let view_mode = match self.view_mode {
            ViewMode::Month => self.generate_month_view(),
            ViewMode::Week => self.generate_week_view(),
            ViewMode::Day => self.generate_day_view(),
            ViewMode::List => self.generate_list_view(),
        }
        .width(Fill)
        .height(Fill)
        .padding(5);

        column![header, horizontal_rule(2), view_mode]
            .width(Fill)
            .height(Fill)
            .into()
    }

    fn generate_month_view(&self) -> Column<Message> {
        let weekdays = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
        let weekday_row = row![row(weekdays.iter().map(|&day| text(day)
            .size(20)
            .width(Fill)
            .align_x(Center)
            .into()))]
        .spacing(5)
        .width(Fill);

        let mut grid = column![].width(Fill).height(Fill).spacing(5);
        let first_day = self.current_date.with_day(1).unwrap();
        let start_offset = first_day.weekday().num_days_from_sunday() as i64;
        let mut current_date = first_day - Duration::days(start_offset);

        for _ in 0..6 {
            let mut week_row = row![].width(Fill).spacing(5);

            for _ in 0..7 {
                let is_current_month = current_date.month() == self.current_date.month();
                let day_text = if is_current_month {
                    text!("{:2}", current_date.day())
                        .size(20)
                        .width(Fill)
                        .height(Fill)
                        .align_x(Center)
                        .align_y(Center)
                } else {
                    text!("{:2}", current_date.day())
                        .size(20)
                        .color([0.6, 0.6, 0.6])
                        .width(Fill)
                        .height(Fill)
                        .align_x(Center)
                        .align_y(Center)
                };
                week_row = week_row.push(day_text).width(Fill);
                current_date += Duration::days(1);
            }
            grid = grid.push(week_row).width(Fill).height(Fill);
        }

        column![weekday_row, grid].width(Fill).height(Fill)
    }

    fn generate_week_view(&self) -> Column<Message> {
        let start_of_week = self.current_date
            - Duration::days(self.current_date.weekday().num_days_from_sunday() as i64);
        let mut week_row = row![].width(Fill).spacing(5);

        for i in 0..7 {
            let day = start_of_week + Duration::days(i);
            week_row = week_row
                .push(text!("{} {:2}", day.format("%a"), day.day()).size(20))
                .width(Fill);
        }

        column![week_row].width(Fill).height(Fill)
    }

    fn generate_day_view(&self) -> Column<Message> {
        column![text(self.current_date.format("%A, %B %d, %Y").to_string()).size(25)]
            .width(Fill)
            .height(Fill)
    }

    fn generate_list_view(&self) -> Column<Message> {
        let mut list = column![].width(Fill).spacing(5);

        for i in 0..30 {
            let day = self.current_date + Duration::days(i);
            list = list
                .push(text!("{}", day.format("%A, %B %d, %Y")).size(20))
                .width(Fill);
        }

        list
    }
}

impl fmt::Display for ViewMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ViewMode::Month => write!(f, "Month"),
            ViewMode::Week => write!(f, "Week"),
            ViewMode::Day => write!(f, "Day"),
            ViewMode::List => write!(f, "List"),
        }
    }
}
