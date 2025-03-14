pub fn get_image(s: String) -> Option<Vec<u8>> {
    match dirs::home_dir() {
        Some(mut path) => {
            path.push(".control_scrapper");
            path.push("control_scrapper");
            path.set_extension("conf");

            let log_path =
                if let Ok(log_path_hyp) = std::fs::read_to_string(path.display().to_string()) {
                    log_path_hyp.lines().next().unwrap().to_string()
                } else {
                    path.pop();
                    path.push("image_log");
                    path.set_extension("log");
                    path.display().to_string()
                };

            crate::log::Log {
                log_path,
                image_path: s.clone(),
            }
            .add_to_log();
            let url = url::Url::parse(&s[..]).unwrap();
            let by = reqwest::blocking::get(url).unwrap().bytes();
            Some(by.unwrap().to_vec())
        }

        None => {
            eprintln!("Impossible to get your home dir!");
            None
        }
    }
}

pub fn store_image(b: Vec<u8>) {
    match dirs::home_dir() {
        Some(mut path) => {
            path.push(".control_scrapper");
            path.push("control_scrapper");
            path.set_extension("conf");
            let _ = std::fs::write(
                if let Ok(log_path_hyp) = std::fs::read_to_string(path.display().to_string()) {
                    log_path_hyp.lines().nth(1).unwrap().to_string()
                } else if let Some(mut image_path) = dirs::picture_dir() {
                    image_path.push("ControlScrapperRes");
                    image_path.push("wallpaper");
                    image_path.set_extension("jpeg");
                    image_path.display().to_string();
                    image_path.display().to_string()
                } else {
                    eprintln!("Impossible to find picture dir");
                    "".to_string()
                },
                b,
            );
        }

        None => {
            println!("Impossible to get your home dir!");
        }
    }
}
