use std::fs::OpenOptions;
use std::io::prelude::*;

use chrono::Local;

#[derive(Default, Debug)]
pub struct Log {
    pub image_path: String,
    pub log_path: String,
}

impl Log {
    pub fn add_to_log(&self) {
        if let Ok(mut file) = OpenOptions::new().append(true).open(self.log_path.clone()) {
            let now = Local::now();

            if let Err(e) = writeln!(
                file,
                "[{}], {}",
                now.to_rfc3339_opts(chrono::format::SecondsFormat::Secs, false),
                self.image_path
            ) {
                eprintln!("Couldn't write log. Err = {}", e);
            }
        } else {
            eprintln!("Couldn't find log_path = {}", self.log_path);
        }
    }
}
