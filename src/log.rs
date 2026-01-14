use chrono::Local;
use std::fs::OpenOptions;
use std::io::prelude::*;

use crate::get_parameter;

#[derive(Default, Debug)]
pub struct Log {
    pub image_url: String,
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
                self.image_url
            ) {
                eprintln!("Couldn't write log. Err = {}", e);
            }
        } else {
            eprintln!("Couldn't find a file at log_path = {}", self.log_path);
        }
    }
}

get_parameter!(
    log_path,
    String,
    {
        match dirs::home_dir() {
            Some(mut path) => {
                path.push(".control_scrapper");
                path.push("image_log");
                path.set_extension("log");
                path.display().to_string()
            }

            None => {
                eprintln!("Impossible to get your home dir!");
                "".to_string()
            }
        }
    },
    to_owned
);
