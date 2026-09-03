use sarwaz::utils::helpers::file_manager;
use sarwaz::utils::logs;

fn main() {
    let welcome_message = file_manager::read_text_file("SomeTxt.txt").expect("file to read");
    logs::setup_logger().expect("Failed to setup logger");
    log::info!("Run application. Welcome message: {}", welcome_message);
}
