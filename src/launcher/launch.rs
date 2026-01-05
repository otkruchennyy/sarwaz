use crate::utils::helpers;
use iced::border::Border;
use iced::widget::{column, container, image, progress_bar, text};
use iced::window;
use iced::{Color, Length, Size, Task};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const WINDOW_WIDTH: f32 = 1000.0;
const WINDOW_HEIGHT: f32 = 540.0;
const BORDER_SIZE: f32 = 10.0;
const IMAGE_HEIGHT: f32 = 500.0;
const TEXT_SIZE: f32 = 16.0;
const PROGRESS_BAR_HEIGHT: f32 = 15.0;
const PROGRESS_BAR_WIDTH: f32 = 800.0;
const DEFAULT_LOADING_TEXT: &str = "Загрузка...";
const DEFAULT_PROGRESS_VALUE: f32 = 0.3;

#[derive(Debug)]
struct LauncherState {
    project_root: PathBuf,
    progress_value: f32,
    loading_text: String,
}

#[derive(Debug, Clone)]
enum Message {
    ProgressUpdated(f32),

    TextUpdated(String),
}

fn init(project_root: PathBuf) -> (LauncherState, Task<Message>) {
    (
        LauncherState {
            project_root,
            progress_value: DEFAULT_PROGRESS_VALUE,
            loading_text: DEFAULT_LOADING_TEXT.to_string(),
        },
        Task::none(),
    )
}

fn update(state: &mut LauncherState, message: Message) -> Task<Message> {
    match message {
        Message::ProgressUpdated(value) => {
            state.progress_value = value.clamp(0.0, 1.0);
            Task::none()
        }
        Message::TextUpdated(new_text) => {
            state.loading_text = new_text;
            Task::none()
        }
    }
}

fn view(state: &LauncherState) -> iced::Element<'_, Message> {
    let image_path = state.project_root.join("assets/image/loading_screen.png");

    column![
        container(image(&*image_path.to_string_lossy()))
            .width(Length::Fill)
            .height(IMAGE_HEIGHT)
            .style(|_theme: &iced::Theme| container::Style {
                border: Border {
                    width: BORDER_SIZE,
                    color: Color::from_rgb(0.1, 0.1, 0.1),
                    radius: 10.0.into(),
                },
                ..Default::default()
            }),
        container(
            column![
                text(&state.loading_text)
                    .size(TEXT_SIZE)
                    .color(Color::WHITE),
                container(progress_bar(0.0..=1.0, state.progress_value).girth(PROGRESS_BAR_HEIGHT))
                    .width(PROGRESS_BAR_WIDTH)
            ]
            .spacing(10)
            .align_x(iced::Alignment::Center)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn custom_theme() -> iced::Theme {
    iced::Theme::custom(
        "Sarwaz Dark".to_string(),
        iced::theme::Palette {
            background: Color::from_rgb(0.1, 0.1, 0.1),
            text: Color::WHITE,
            primary: Color::from_rgb(0.5, 0.5, 0.5),
            success: Color::from_rgb(0.0, 1.0, 0.0),
            danger: Color::from_rgb(1.0, 0.0, 0.0),
            warning: Color::from_rgb(1.0, 0.6, 0.0),
        },
    )
}

pub fn launcher() -> iced::Result {
    let project_root = helpers::get_project_root();

    iced::application(move || init(project_root.clone()), update, view)
        .title("Sarwaz launcher")
        .window(window::Settings {
            decorations: false,
            transparent: true,
            size: Size::new(WINDOW_WIDTH + BORDER_SIZE, WINDOW_HEIGHT + BORDER_SIZE),
            position: window::Position::Centered,
            ..window::Settings::default()
        })
        .theme(custom_theme())
        .run()
}
