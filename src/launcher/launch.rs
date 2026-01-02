use iced::widget::{button, row, text, Row};
use iced::{window, Size};

pub fn launcher() -> iced::Result {
    iced::application(init, update, view)
        .window(window::Settings {
            decorations: false,
            transparent: true,
            size: Size::new(400.0, 200.0),
            ..window::Settings::default()
        })
        .title("Sarwaz launcher") // ← Заголовок задаётся здесь!
        .run()
}

fn init() -> (u64, iced::Task<Message>) {
    (0, iced::Task::none())
}

#[derive(Debug, Clone)]
enum Message {}

fn update(state: &mut u64, message: Message) {
    match message {}
}

fn view(_state: &u64) -> Row<Message> {
    row![text("launcher")]
}
