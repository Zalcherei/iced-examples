use iced::{
    widget::{button, column, row, text, text_input, vertical_space},
    Center, Element, Task, Theme,
};

fn main() -> iced::Result {
    iced::application("Calculator - Iced", Calculator::update, Calculator::view)
        .theme(|_| Theme::Dark)
        .run_with(Calculator::new)
}

struct Calculator {
    input: String,
    result: String,
    operator: Option<Operator>,
    operand: Option<f64>,
    angle_mode: AngleMode,
}

#[derive(Debug, Clone)]
enum Message {
    Input(String),
    Clear,
    Calculate,
    OperatorPressed(Operator),
    TrigFunctionPressed(TrigFunction),
    LogFunctionPressed(LogFunction),
    Exponentiate,
    SquareRoot,
    CubeRoot,
    ToggleAngleMode,
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponentiate,
}

#[derive(Debug, Clone, Copy)]
enum TrigFunction {
    Sine,
    Cosine,
    Tangent,
}

#[derive(Debug, Clone, Copy)]
enum LogFunction {
    Log10,
    Ln,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum AngleMode {
    Degrees,
    #[default]
    Radians,
}

impl Calculator {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                input: String::new(),
                result: String::new(),
                operator: None,
                operand: None,
                angle_mode: AngleMode::Radians,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Input(value) => self.input.push_str(&value),
            Message::Clear => self.clear(),
            Message::Calculate => self.calculate(),
            Message::OperatorPressed(op) => self.handle_operator(op),
            Message::TrigFunctionPressed(trig_fn) => self.apply_trig_function(trig_fn),
            Message::LogFunctionPressed(log_fn) => self.apply_log_function(log_fn),
            Message::Exponentiate => self.prepare_exponentiation(),
            Message::SquareRoot => self.apply_root(2.0),
            Message::CubeRoot => self.apply_root(3.0),
            Message::ToggleAngleMode => self.toggle_angle_mode(),
        }
        Task::none()
    }

    fn clear(&mut self) {
        self.input.clear();
        self.result.clear();
        self.operand = None;
        self.operator = None;
    }

    fn calculate(&mut self) {
        if let (Some(operator), Some(operand)) = (self.operator, self.operand) {
            if let Ok(current_value) = self.input.parse::<f64>() {
                let result = match operator {
                    Operator::Add => operand + current_value,
                    Operator::Subtract => operand - current_value,
                    Operator::Multiply => operand * current_value,
                    Operator::Divide => operand / current_value,
                    Operator::Exponentiate => operand.powf(current_value),
                };
                self.result = result.to_string();
                self.input.clear();
                self.operand = Some(result);
                self.operator = None;
            }
        }
    }

    fn handle_operator(&mut self, op: Operator) {
        if self.operand.is_none() {
            self.operand = self.input.parse::<f64>().ok();
        } else {
            self.calculate();
        }
        self.input.clear();
        self.operator = Some(op);
    }

    fn apply_trig_function(&mut self, trig_fn: TrigFunction) {
        if let Ok(angle) = self.input.parse::<f64>() {
            let angle_radians = if self.angle_mode == AngleMode::Degrees {
                angle.to_radians()
            } else {
                angle
            };
            let result = match trig_fn {
                TrigFunction::Sine => angle_radians.sin(),
                TrigFunction::Cosine => angle_radians.cos(),
                TrigFunction::Tangent => angle_radians.tan(),
            };
            self.result = result.to_string();
            self.input.clear();
        }
    }

    fn apply_log_function(&mut self, log_fn: LogFunction) {
        if let Ok(value) = self.input.parse::<f64>() {
            let result = match log_fn {
                LogFunction::Log10 => value.log10(),
                LogFunction::Ln => value.ln(),
            };
            self.result = result.to_string();
            self.input.clear();
        }
    }

    fn prepare_exponentiation(&mut self) {
        if let Ok(base) = self.input.parse::<f64>() {
            self.operand = Some(base);
            self.operator = Some(Operator::Exponentiate);
            self.input.clear();
        }
    }

    fn apply_root(&mut self, degree: f64) {
        if let Ok(value) = self.input.parse::<f64>() {
            self.result = value.powf(1.0 / degree).to_string();
            self.input.clear();
        }
    }

    fn toggle_angle_mode(&mut self) {
        self.angle_mode = match self.angle_mode {
            AngleMode::Degrees => AngleMode::Radians,
            AngleMode::Radians => AngleMode::Degrees,
        };
    }

    fn view(&self) -> Element<Message> {
        let button_size = 60;
        let create_button = |label: &'static str, message: Message| {
            button(text(label).align_x(Center).align_y(Center))
                .padding(10)
                .width(button_size)
                .height(button_size)
                .on_press(message)
        };

        let clear_button = button(text("C").align_x(Center).align_y(Center))
            .padding(10)
            .width(button_size)
            .height(button_size)
            .on_press(Message::Clear);

        let angle_mode_button = button(
            text(match self.angle_mode {
                AngleMode::Degrees => "Deg",
                AngleMode::Radians => "Rad",
            })
            .align_x(Center)
            .align_y(Center),
        )
        .padding(10)
        .width(button_size)
        .height(button_size)
        .on_press(Message::ToggleAngleMode);

        let buttons = vec![
            ("7", Message::Input("7".into())),
            ("8", Message::Input("8".into())),
            ("9", Message::Input("9".into())),
            ("/", Message::OperatorPressed(Operator::Divide)),
            ("4", Message::Input("4".into())),
            ("5", Message::Input("5".into())),
            ("6", Message::Input("6".into())),
            ("*", Message::OperatorPressed(Operator::Multiply)),
            ("1", Message::Input("1".into())),
            ("2", Message::Input("2".into())),
            ("3", Message::Input("3".into())),
            ("-", Message::OperatorPressed(Operator::Subtract)),
            ("0", Message::Input("0".into())),
            (".", Message::Input(".".into())),
            ("=", Message::Calculate),
            ("+", Message::OperatorPressed(Operator::Add)),
            ("sin", Message::TrigFunctionPressed(TrigFunction::Sine)),
            ("cos", Message::TrigFunctionPressed(TrigFunction::Cosine)),
            ("tan", Message::TrigFunctionPressed(TrigFunction::Tangent)),
            ("log", Message::LogFunctionPressed(LogFunction::Log10)),
            ("ln", Message::LogFunctionPressed(LogFunction::Ln)),
            ("x^y", Message::Exponentiate),
            ("√", Message::SquareRoot),
            ("cbrt", Message::CubeRoot),
        ];

        let button_grid = buttons.chunks(4).fold(column![].spacing(10), |col, row| {
            let button_row = row.iter().fold(row![].spacing(10), |r, (label, msg)| {
                r.push(create_button(label, msg.clone()))
            });
            col.push(button_row)
        });

        column![
            text(&self.result).size(18),
            text_input("0", &self.input).size(18),
            vertical_space(),
            row![clear_button, angle_mode_button].spacing(10),
            vertical_space(),
            button_grid
        ]
        .padding(20)
        .align_x(Center)
        .into()
    }
}
