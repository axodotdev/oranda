use console::Color::{Green, Magenta, Red, White, Yellow};
use console::Style;

pub enum MessageType {
    Success,
    Info,
    Debug,
    Warning,
    Error,
}

pub struct Message {
    mtype: MessageType,
    msg: String,
}

impl Message {
    pub fn new(mtype: MessageType, msg: &str) -> Self {
        Message { mtype, msg: msg.to_string() }
    }

    pub fn print_and_log(&self) {
        self.log();
        self.print();
    }

    pub fn log(&self) {
        match &self.mtype {
            MessageType::Success => {
                tracing::info!(self.msg);
            }
            MessageType::Info => {
                tracing::info!(self.msg);
            }
            MessageType::Debug => {
                tracing::debug!(self.msg);
            }
            MessageType::Warning => {
                tracing::warn!(self.msg);
            }
            MessageType::Error => {
                tracing::error!(self.msg);
            }
        }
    }

    pub fn print(&self) {
        println!("{}", &self.style());
    }

    fn style(&self) -> String {
        let warn_icon = Style::new().bold().fg(Yellow).apply_to("⚠");
        let check_icon = Style::new().bold().fg(Green).apply_to("✓");
        let x_icon = Style::new().bold().fg(Red).apply_to("✗");
        let arrow_icon = Style::new().bold().fg(White).apply_to("↪");
        match self.mtype {
            MessageType::Success => {
                let style = Style::new().bold().fg(Green);
                format!("{} SUCCESS: {}", check_icon, style.apply_to(&self.msg))
            }
            MessageType::Info => {
                let style = Style::new().bold().fg(White);
                format!("{} INFO: {}", arrow_icon, style.apply_to(&self.msg))
            }
            MessageType::Debug => {
                let style = Style::new().bold().fg(Magenta);
                format!("{} DEBUG: {}", arrow_icon, style.apply_to(&self.msg))
            }
            MessageType::Warning => {
                let style = Style::new().bold().fg(Yellow);
                format!("{} WARNING: {}", warn_icon, style.apply_to(&self.msg))
            }
            MessageType::Error => {
                let style = Style::new().bold().fg(Red);
                format!("{} ERROR: {}", x_icon, style.apply_to(&self.msg))
            }
        }
    }
}
