use crate::utils::helpers;
use iced::widget::{row, text};
use iced::{window, Color, Size, Task};
use std::path::PathBuf;

pub fn launcher() -> iced::Result {
    let project_root = helpers::get_project_root();

    iced::application(move || init(project_root.clone()), update, view)
        .title("Sarwaz launcher")
        .window(window::Settings {
            decorations: false,
            transparent: false,
            size: Size::new(1006.0, 506.0),
            ..window::Settings::default()
        })
        .theme(custom_theme())
        .run()
}

#[derive(Debug)]
struct LauncherState {
    counter: u64,
}

// Сообщения приложения
#[derive(Debug, Clone)]
enum Message {
    // other message
}

fn init(project_root: PathBuf) -> (LauncherState, Task<Message>) {
    let info = CreatePropetsJson::new(&project_root);

    (
        LauncherState {
            counter: 0,
            // инициализация других полей здесь
        },
        Task::none(),
    )
}

fn update(state: &mut LauncherState, message: Message) -> Task<Message> {
    match message {
        // обработка сообщений тут
        _ => Task::none(),
    }
}

// render UI
fn view(state: &LauncherState) -> iced::Element<'_, Message> {
    row![text("launcher")].into()
}

// Custom theme
fn custom_theme() -> iced::Theme {
    iced::Theme::custom(
        "Sarwaz Dark".to_string(),
        iced::theme::Palette {
            background: Color::from_rgb(0.1, 0.1, 0.1),
            text: Color::WHITE,
            primary: Color::from_rgb(0.5, 0.5, 1.0),
            success: Color::from_rgb(0.0, 1.0, 0.0),
            danger: Color::from_rgb(1.0, 0.0, 0.0),
            warning: Color::from_rgb(1.0, 0.6, 0.0),
        },
    )
}

struct CreatePropetsJson<'a> {
    project_root: &'a PathBuf,
}

impl<'a> CreatePropetsJson<'a> {
    fn new(project_root: &'a PathBuf) -> Self {
        CreatePropetsJson { project_root }
    }
}
