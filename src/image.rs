use crate::get_parameter;
use crate::log::get_log_path;

pub fn get_image(s: String) -> Option<Vec<u8>> {
    println!("{}", s);
    match dirs::home_dir() {
        Some(mut path) => {
            path.push(".control_scrapper");
            path.push("control_scrapper");
            path.set_extension("conf");

            let log_path = get_log_path();

            crate::log::Log {
                log_path,
                image_url: s.clone(),
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

pub fn store_image(image_bits: Vec<u8>) -> bool {
    let image_path = get_image_path();
    if !image_path.is_empty() {
        let _ = std::fs::write(image_path, image_bits);
        true
    } else {
        false
    }
}

get_parameter!(
    image_path,
    String,
    {
        match dirs::picture_dir() {
            Some(mut path) => {
                path.push("ControlScrapperRes");
                path.push("wallpaper");
                path.set_extension("jpeg");
                path.display().to_string();
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
