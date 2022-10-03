use std::io::Write;
use std::sync::Mutex;

use console::Term;
use lazy_static::lazy_static;
use serde::Serialize;
use tracing::error;

type ReportErrorFunc = dyn Fn(&miette::Report) + Send + Sync + 'static;

// XXX: We might be able to get rid of this `lazy_static` after 1.63 due to
// `const Mutex::new` being stabilized.
lazy_static! {
    static ref REPORT_ERROR: Mutex<Option<Box<ReportErrorFunc>>> = Mutex::new(None);
}

#[derive(Serialize)]
pub struct Report {}

impl Report {
    pub fn print_human(&self, _out: &mut Term) -> Result<(), std::io::Error> {
        // Nothing to report on success to humans? (yay!)
        Ok(())
    }

    pub fn print_json(&self, out: &mut Term) -> Result<(), std::io::Error> {
        let data: String = serde_json::to_string_pretty(self)?;
        writeln!(out, "{}", data)?;
        Ok(())
    }

    pub fn as_json() {
        *REPORT_ERROR.lock().unwrap() = Some(Box::new(move |error| {
            // Manually invoke JSONReportHandler to format the error as a report
            // to out_.
            let mut report = String::new();
            miette::JSONReportHandler::new()
                .render_report(&mut report, error.as_ref())
                .unwrap();
            writeln!(&mut Term::stdout(), r#"{{"error": {}}}"#, report).unwrap();
        }));
    }

    pub fn error(error: &miette::Report) {
        {
            let guard = REPORT_ERROR.lock().unwrap();
            if let Some(do_report) = &*guard {
                do_report(error);
                return;
            }
        }
        error!("{:?}", error);
    }
}
