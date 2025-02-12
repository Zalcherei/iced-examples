# Rust Iced GUI timer

Timer with Rust Iced GUI 0.13. features:

- Name Field
- Hours, Minutes, Seconds Field - Editable fields. Default should be one minute.
- Start and Stop button
- Delete timer button
- New timer button

[dependencies]
iced = { version = "0.13.1", features = ["advanced", "tokio", "debug"] }
uuid = { version = "1.12.1", features = ["v4"] }
