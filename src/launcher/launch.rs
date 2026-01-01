use iced::widget::{column, Column};

pub fn launcher() -> iced::Result {
    iced::application(init, update, view).run()
}

// 1. Функция инициализации (заменяет `new`)
fn init() -> (u64, iced::Task<Message>) {
    (0, iced::Task::none()) // Возвращаем начальное состояние и фоновую задачу
}

// 2. Перечисление сообщений
#[derive(Debug, Clone)]
enum Message {}

// 3. Функция обновления (заменяет `update`)
fn update(state: &mut u64, message: Message) {
    match message {}
}

// 4. Функция отображения (заменяет `view`)
fn view(state: &u64) -> Column<Message> {
    column![]
}
