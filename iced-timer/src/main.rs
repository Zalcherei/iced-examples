use iced::{
    time,
    widget::{button, column, container, row, text_input},
    Element, Subscription, Task, Theme,
};
use std::time::Duration;
use uuid::Uuid;

fn main() -> iced::Result {
    iced::application("Timer - Iced", TimerApp::update, TimerApp::view)
        .theme(|_| Theme::Dark)
        .subscription(TimerApp::subscription)
        .run_with(TimerApp::new)
}

#[derive(Debug, Clone)]
struct Timer {
    id: Uuid,
    name: String,
    hours: u32,
    minutes: u32,
    seconds: u32,
    is_running: bool,
}

#[derive(Debug, Clone)]
enum Message {
    NameChanged(Uuid, String),
    HoursChanged(Uuid, u32),
    MinutesChanged(Uuid, u32),
    SecondsChanged(Uuid, u32),
    StartTimer(Uuid),
    StopTimer(Uuid),
    DeleteTimer(Uuid),
    AddTimer,
    Tick,
}

struct TimerApp {
    timers: Vec<Timer>,
}

impl TimerApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                timers: vec![Timer {
                    id: Uuid::new_v4(),
                    name: String::from("Timer 1"),
                    hours: 0,
                    minutes: 1,
                    seconds: 0,
                    is_running: false,
                }],
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NameChanged(id, name) => {
                if let Some(timer) = self.timers.iter_mut().find(|t| t.id == id) {
                    timer.name = name;
                }
            }
            Message::HoursChanged(id, hours) => {
                if let Some(timer) = self.timers.iter_mut().find(|t| t.id == id) {
                    timer.hours = hours;
                }
            }
            Message::MinutesChanged(id, minutes) => {
                if let Some(timer) = self.timers.iter_mut().find(|t| t.id == id) {
                    timer.minutes = minutes;
                }
            }
            Message::SecondsChanged(id, seconds) => {
                if let Some(timer) = self.timers.iter_mut().find(|t| t.id == id) {
                    timer.seconds = seconds;
                }
            }
            Message::StartTimer(id) => {
                if let Some(timer) = self.timers.iter_mut().find(|t| t.id == id) {
                    timer.is_running = true;
                }
            }
            Message::StopTimer(id) => {
                if let Some(timer) = self.timers.iter_mut().find(|t| t.id == id) {
                    timer.is_running = false;
                }
            }
            Message::DeleteTimer(id) => {
                self.timers.retain(|t| t.id != id);
            }
            Message::AddTimer => {
                self.timers.push(Timer {
                    id: Uuid::new_v4(),
                    name: format!("Timer {}", self.timers.len() + 1),
                    hours: 0,
                    minutes: 1,
                    seconds: 0,
                    is_running: false,
                });
            }
            Message::Tick => {
                for timer in &mut self.timers {
                    if timer.is_running {
                        if timer.seconds > 0 {
                            timer.seconds -= 1;
                        } else if timer.minutes > 0 {
                            timer.minutes -= 1;
                            timer.seconds = 59;
                        } else if timer.hours > 0 {
                            timer.hours -= 1;
                            timer.minutes = 59;
                            timer.seconds = 59;
                        } else {
                            timer.is_running = false;
                        }
                    }
                }
            }
        }
        Task::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }

    fn view(&self) -> Element<Message> {
        let mut column = column![].padding(10).spacing(10);

        for timer in &self.timers {
            let content = container(
                column![
                    text_input(&timer.name, &timer.name)
                        .on_input(move |name| Message::NameChanged(timer.id, name)),
                    row![
                        text_input(&timer.hours.to_string(), &timer.hours.to_string()).on_input(
                            move |hours| {
                                if let Ok(hours) = hours.parse() {
                                    Message::HoursChanged(timer.id, hours)
                                } else {
                                    Message::HoursChanged(timer.id, 0)
                                }
                            }
                        ),
                        text_input(&timer.minutes.to_string(), &timer.minutes.to_string())
                            .on_input(move |minutes| {
                                if let Ok(minutes) = minutes.parse() {
                                    Message::MinutesChanged(timer.id, minutes)
                                } else {
                                    Message::MinutesChanged(timer.id, 0)
                                }
                            }),
                        text_input(&timer.seconds.to_string(), &timer.seconds.to_string())
                            .on_input(move |seconds| {
                                if let Ok(seconds) = seconds.parse() {
                                    Message::SecondsChanged(timer.id, seconds)
                                } else {
                                    Message::SecondsChanged(timer.id, 0)
                                }
                            }),
                    ]
                    .spacing(5),
                    row![
                        if timer.is_running {
                            button("Stop").on_press(Message::StopTimer(timer.id))
                        } else {
                            button("Start").on_press(Message::StartTimer(timer.id))
                        },
                        button("Delete").on_press(Message::DeleteTimer(timer.id)),
                    ]
                    .spacing(5)
                ]
                .spacing(5),
            )
            .padding(10)
            .style(container::bordered_box);

            column = column.push(content);
        }

        column = column.push(button("Add Timer").on_press(Message::AddTimer));

        column.into()
    }
}
