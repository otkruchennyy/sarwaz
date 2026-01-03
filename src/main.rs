use sarwaz::launcher::launch as launcherInit;
use sarwaz::utils::helpers;
use std::path::PathBuf;

fn main() {
    let project_root: PathBuf = helpers::get_project_root();

    if let Err(e) = launcherInit::launcher() {
        eprintln!("Ошибка запуска приложения: {}", e);
        std::process::exit(1);
    }
}
