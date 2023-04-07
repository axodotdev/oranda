use clap::ValueEnum;
use console::Color::{Cyan, Green, Magenta, Red, White, Yellow};
use console::Style;

pub enum MessageType {
    Success,
    Info,
    Hint,
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
        Message {
            mtype,
            msg: msg.to_string(),
        }
    }

    pub fn print(&self) {
        eprintln!("{}", &self.style());
    }

    fn style(&self) -> String {
        let warn_icon = Style::new().bold().fg(Yellow).apply_to("⚠");
        let check_icon = Style::new().bold().fg(Green).apply_to("✓");
        let x_icon = Style::new().bold().fg(Red).apply_to("✗");
        let arrow_icon = Style::new().bold().fg(White).apply_to("↪");
        match self.mtype {
            MessageType::Success => {
                let style = Style::new().bold().fg(Green);
                format!(
                    "{} >o_o< SUCCESS: {}",
                    check_icon,
                    style.apply_to(&self.msg)
                )
            }
            MessageType::Info => {
                let style = Style::new().bold().fg(White);
                format!("{} >o_o< INFO: {}", arrow_icon, style.apply_to(&self.msg))
            }
            MessageType::Hint => {
                let style = Style::new().bold().fg(Cyan);
                format!("{} >o_o< HINT: {}", arrow_icon, style.apply_to(&self.msg))
            }
            MessageType::Debug => {
                let style = Style::new().bold().fg(Magenta);
                format!("{} >o_o< DEBUG: {}", arrow_icon, style.apply_to(&self.msg))
            }
            MessageType::Warning => {
                let style = Style::new().bold().fg(Yellow);
                format!("{} >o_o< WARNING: {}", warn_icon, style.apply_to(&self.msg))
            }
            MessageType::Error => {
                let style = Style::new().bold().fg(Red);
                format!("{} >o_o< ERROR: {}", x_icon, style.apply_to(&self.msg))
            }
        }
    }
}

/// Style of output we should produce
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputFormat {
    /// Human-readable output
    Human,
    /// Machine-readable JSON output
    Json,
}
