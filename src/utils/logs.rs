use fern::Dispatch;
use log::LevelFilter;

pub fn setup_logger() -> Result<(), fern::InitError> {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} - {} - {}",
                chrono::Local::now().format("%H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("app.log")?)
        .apply()?;
    Ok(())
}

/* TODO:
Писать логи в
     Windows: %APPDATA% или %LOCALAPPDATA%
     Linux: ~/.local/share/
     macOS: ~/Library/Application Support/
*/
