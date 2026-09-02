use sarwaz::utils::helpers;
use sarwaz::utils::logs;

fn main() {
    let welcome_message = helpers::read_text_file("SomeTxt.txt").expect("file to read");
    logs::setup_logger().expect("Failed to setup logger");
    log::info!("Run application. Welcome message: {}", welcome_message);
}
