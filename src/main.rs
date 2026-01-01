use sarwaz::launcher::launch as launcherInit;

fn main() {
    if let Err(e) = launcherInit::launcher() {
        eprintln!("Ошибка запуска приложения: {}", e);
        std::process::exit(1);
    }
}
