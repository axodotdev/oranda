use console::Color::{Green, Magenta, Red, White, Yellow};
use console::Style;

pub enum MessageType {
    Success,
    Info,
    Debug,
    Warning,
    Error,
}

pub fn build(msgtype: MessageType, msg: &str) -> String {
    let warn_icon = Style::new().bold().fg(Yellow).apply_to("⚠");
    let check_icon = Style::new().bold().fg(Green).apply_to("✓");
    let x_icon = Style::new().bold().fg(Red).apply_to("✗");
    let arrow_icon = Style::new().bold().fg(White).apply_to("↪");
    match msgtype {
        MessageType::Success => {
            let style = Style::new().bold().fg(Green);
            format!("{} SUCCESS: {}", check_icon, style.apply_to(msg))
        }
        MessageType::Info => {
            let style = Style::new().bold().fg(White);
            format!("{} INFO: {}", arrow_icon, style.apply_to(msg))
        }
        MessageType::Debug => {
            let style = Style::new().bold().fg(Magenta);
            format!("{} DEBUG: {}", arrow_icon, style.apply_to(msg))
        }
        MessageType::Warning => {
            let style = Style::new().bold().fg(Yellow);
            format!("{} WARNING: {}", warn_icon, style.apply_to(msg))
        }
        MessageType::Error => {
            let style = Style::new().bold().fg(Red);
            format!("{} ERROR: {}", x_icon, style.apply_to(msg))
        }
    }
}
