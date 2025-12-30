## **Installation**

Clone repository

## **Usage**

Pixel art drawing application

## **Structure**

sarwaz/ <br>
├── src/ <br>
│   ├── .../ <br>
│   │   ├── ... <br>
│   │   └── ... <br>
│   └── main.rs <br>
├── README.md <br>
└── LICENSE.md

Reference structure

sarwaz/                                 # Workspace Cargo.toml
├── pixel_core/                         # Крейт с чистой логикой
│   ├── src/
│   │   ├── lib.rs
│   │   ├── color/                      # Вся логика цвета
│   │   │   ├── mod.rs                  # (Re-)export модулей
│   │   │   ├── palette.rs              # Palette, ColorModel
│   │   │   ├── indexed.rs              # IndexedColor
│   │   │   └── rgba.rs                 # RgbaColor (на будущее)
│   │   ├── document/                   # Модель данных
│   │   │   ├── mod.rs
│   │   │   ├── layer.rs                # Layer { name, pixels, visible, opacity }
│   │   │   └── animation.rs            # Frame, Timeline (на будущее)
│   │   ├── command/                    # Паттерн Команда для undo/redo
│   │   │   ├── mod.rs
│   │   │   ├── history.rs              # History { commands, cursor }
│   │   │   ├── trait.rs                # trait Command
│   │   │   ├── basic.rs                # SetPixel, FillArea, etc.
│   │   │   └── layer.rs                # AddLayer, MergeLayer (на будущее)
│   │   ├── tool/                       # Инструменты
│   │   │   ├── mod.rs
│   │   │   ├── trait.rs                # trait Tool
│   │   │   ├── pencil.rs
│   │   │   ├── eraser.rs
│   │   │   └── bucket.rs               # BucketFill (на будущее)
│   │   ├── editor.rs                   # Главная структура состояния приложения
│   │   └── export/                     # Экспорт в файлы (на будущее)
│   │       ├── mod.rs
│   │       ├── png.rs
│   │       └── gif.rs
├── pixel_ui/                           # Крейт с интерфейсом на Iced
│   ├── src/
│   │   ├── main.rs                     # Точка входа, настройка Iced
│   │   ├── app.rs                      # Struct App { editor, .. }, enum Message
│   │   ├── canvas.rs                   # Кастомный виджет CanvasWidget
│   │   └── widgets/                    # Остальные виджеты UI
│   │       ├── mod.rs
│   │       ├── toolbar.rs
│   │       ├── color_picker.rs
│   │       └── layer_list.rs
└── Cargo.toml                          # Workspace определение

## **License**

Licensed under the Apache License 2.0 - see the [LICENSE](LICENSE.md) file for details.

## **Acknowledgements**

This project is made possible thanks to the incredible work of the [Rust](https://www.rust-lang.org/)<img src="https://rustacean.net/assets/rustacean-flat-happy.png" width="22" height="16"> and [Iced](https://iced.rs/)<img src="https://iced.rs/logo.svg" width="16" height="16"> development communities.

## **Author | Автор**

[otkruchenyy](https://github.com/otkruchenyy)
[tg_channel](https://t.me/+BTSVg57miuhiNDQy)
