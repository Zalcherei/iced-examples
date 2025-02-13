use iced::{
    widget::{button, column, container, row, text, text_input, Button, Column},
    Center, Element, Fill, Font, Task, Theme,
};
use std::f64::consts::{E, PI};

fn main() -> iced::Result {
    iced::application("Calculator v2 - Iced", Calculator::update, Calculator::view)
        .theme(|_| Theme::Dark)
        .default_font(Font::MONOSPACE)
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
    ToggleAngleMode,
    Factorial,
    Square,
    Cube,
    Reciprocal,
    RootY,
    Exponential,
    Euler,
    EE,
    Percentage,
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
            Message::ToggleAngleMode => self.toggle_angle_mode(),
            Message::Factorial => {
                if let Ok(n) = self.input.parse::<u64>() {
                    self.result = Self::factorial(n).to_string();
                    self.input.clear();
                }
            }
            Message::Square => {
                if let Ok(n) = self.input.parse::<f64>() {
                    self.result = (n * n).to_string();
                    self.input.clear();
                }
            }
            Message::Cube => {
                if let Ok(n) = self.input.parse::<f64>() {
                    self.result = (n * n * n).to_string();
                    self.input.clear();
                }
            }
            Message::Reciprocal => {
                if let Ok(n) = self.input.parse::<f64>() {
                    if n != 0.0 {
                        self.result = (1.0 / n).to_string();
                    } else {
                        self.result = "Error".into();
                    }
                    self.input.clear();
                }
            }
            Message::RootY => {
                if let (Some(base), Ok(root)) = (self.operand, self.input.parse::<f64>()) {
                    self.result = base.powf(1.0 / root).to_string();
                    self.input.clear();
                    self.operand = None;
                }
            }
            Message::Exponential => {
                if let Ok(n) = self.input.parse::<f64>() {
                    self.result = n.exp().to_string();
                    self.input.clear();
                }
            }
            Message::Euler => {
                self.result = E.to_string();
            }
            Message::EE => {
                self.input.push_str("e");
            }
            Message::Percentage => {
                if let Ok(n) = self.input.parse::<f64>() {
                    self.result = (n / 100.0).to_string();
                    self.input.clear();
                }
            }
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

    fn factorial(n: u64) -> u64 {
        (1..=n).product()
    }

    fn scientific_buttons(&self) -> Column<Message> {
        column![
            row![
                button(
                    text(match self.angle_mode {
                        AngleMode::Degrees => "Deg",
                        AngleMode::Radians => "Rad",
                    })
                    .size(24)
                    .align_x(Center)
                    .align_y(Center)
                )
                .width(150)
                .height(50)
                .on_press(Message::ToggleAngleMode),
                calc_button("(", Message::Input("(".into())),
                calc_button(")", Message::Input(")".into()))
            ],
            row![
                calc_button("sin", Message::TrigFunctionPressed(TrigFunction::Sine)),
                calc_button("cos", Message::TrigFunctionPressed(TrigFunction::Cosine)),
                calc_button("tan", Message::TrigFunctionPressed(TrigFunction::Tangent)),
                calc_button("π", Message::Input(PI.to_string()))
            ],
            row![
                calc_button("x!", Message::Factorial),
                calc_button("x²", Message::Square),
                calc_button("x³", Message::Cube),
                calc_button("xy", Message::Exponentiate)
            ],
            row![
                calc_button("1/x", Message::Reciprocal),
                calc_button("√x", Message::SquareRoot),
                calc_button("x√y", Message::RootY),
                calc_button("EE", Message::EE)
            ],
            row![
                calc_button("log", Message::LogFunctionPressed(LogFunction::Log10)),
                calc_button("ln", Message::LogFunctionPressed(LogFunction::Ln)),
                calc_button("eˣ", Message::Exponential),
                calc_button("e", Message::Euler)
            ]
        ]
    }

    fn basic_buttons(&self) -> Column<Message> {
        column![
            row![
                button(text("C").size(24).align_x(Center).align_y(Center))
                    .width(150)
                    .height(50)
                    .on_press(Message::Clear),
                calc_button("%", Message::Percentage),
                calc_button("÷", Message::OperatorPressed(Operator::Divide))
            ],
            row![
                calc_button("7", Message::Input("7".into())),
                calc_button("8", Message::Input("8".into())),
                calc_button("9", Message::Input("9".into())),
                calc_button("×", Message::OperatorPressed(Operator::Multiply))
            ],
            row![
                calc_button("4", Message::Input("4".into())),
                calc_button("5", Message::Input("5".into())),
                calc_button("6", Message::Input("6".into())),
                calc_button("−", Message::OperatorPressed(Operator::Subtract))
            ],
            row![
                calc_button("1", Message::Input("1".into())),
                calc_button("2", Message::Input("2".into())),
                calc_button("3", Message::Input("3".into())),
                calc_button("+", Message::OperatorPressed(Operator::Add))
            ],
            row![
                button(text("0").size(24).align_x(Center).align_y(Center))
                    .width(150)
                    .height(50)
                    .on_press(Message::Input("0".into())),
                calc_button(".", Message::Input(".".into())),
                calc_button("=", Message::Calculate)
            ]
        ]
    }

    fn view(&self) -> Element<Message> {
        let result_display = text_input("0", &self.input).size(24).width(602);

        container(row![column![
            text(&self.result).size(24),
            result_display,
            row![self.scientific_buttons(), self.basic_buttons()].spacing(2)
        ]])
        .width(Fill)
        .height(Fill)
        .align_y(Center)
        .align_x(Center)
        .into()
    }
}

fn calc_button(label: &str, message: Message) -> Button<Message> {
    button(text(label).size(24).align_x(Center).align_y(Center))
        .width(75)
        .height(50)
        .on_press(message)
}

/*
/// Testing the calculator
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(Calculator::factorial(0), 1);
        assert_eq!(Calculator::factorial(5), 120);
        assert_eq!(Calculator::factorial(10), 3628800);
    }

    #[test]
    fn test_apply_trig_function() {
        let mut calc = Calculator::new().0;

        // Test sine function in radians
        calc.input = "0".to_string();
        calc.apply_trig_function(TrigFunction::Sine);
        assert_eq!(calc.result, "0");

        // Test cosine function in degrees
        calc.angle_mode = AngleMode::Degrees;
        calc.input = "90".to_string();
        calc.apply_trig_function(TrigFunction::Cosine);
        assert_eq!(calc.result, "0");

        // Test tangent function in radians
        calc.angle_mode = AngleMode::Radians;
        calc.input = "0".to_string();
        calc.apply_trig_function(TrigFunction::Tangent);
        assert_eq!(calc.result, "0");
    }

    #[test]
    fn test_apply_log_function() {
        let mut calc = Calculator::new().0;

        // Test log10
        calc.input = "100".to_string();
        calc.apply_log_function(LogFunction::Log10);
        assert_eq!(calc.result, "2");

        // Test natural log
        calc.input = E.to_string();
        calc.apply_log_function(LogFunction::Ln);
        assert_eq!(calc.result, "1");
    }

    #[test]
    fn test_update_input() {
        let mut calc = Calculator::new().0;

        // Simulate user input
        calc.update(Message::Input("5".to_string()));
        assert_eq!(calc.input, "5");

        calc.update(Message::Input("3".to_string()));
        assert_eq!(calc.input, "53");
    }

    #[test]
    fn test_update_clear() {
        let mut calc = Calculator::new().0;

        // Simulate user input and then clear
        calc.update(Message::Input("123".to_string()));
        calc.update(Message::Clear);
        assert_eq!(calc.input, "");
        assert_eq!(calc.result, "");
        assert_eq!(calc.operand, None);
        // assert_eq!(calc.operator, None);
    }

    #[test]
    fn test_update_calculate() {
        let mut calc = Calculator::new().0;

        // Simulate 5 + 3
        calc.update(Message::Input("5".to_string()));
        calc.update(Message::OperatorPressed(Operator::Add));
        calc.update(Message::Input("3".to_string()));
        calc.update(Message::Calculate);
        assert_eq!(calc.result, "8");
    }

    #[test]
    fn test_update_toggle_angle_mode() {
        let mut calc = Calculator::new().0;

        // Initial mode is Radians
        assert_eq!(calc.angle_mode, AngleMode::Radians);

        // Toggle to Degrees
        calc.update(Message::ToggleAngleMode);
        assert_eq!(calc.angle_mode, AngleMode::Degrees);

        // Toggle back to Radians
        calc.update(Message::ToggleAngleMode);
        assert_eq!(calc.angle_mode, AngleMode::Radians);
    }

    #[test]
    fn test_division_by_zero() {
        let mut calc = Calculator::new().0;

        // Simulate 1 / 0
        calc.update(Message::Input("1".to_string()));
        calc.update(Message::OperatorPressed(Operator::Divide));
        calc.update(Message::Input("0".to_string()));
        calc.update(Message::Calculate);
        assert_eq!(calc.result, "Error");
    }

    #[test]
    fn test_invalid_input() {
        let mut calc = Calculator::new().0;

        // Simulate invalid input
        calc.update(Message::Input("abc".to_string()));
        calc.update(Message::Calculate);
        assert_eq!(calc.result, "");
    }
}
*/
