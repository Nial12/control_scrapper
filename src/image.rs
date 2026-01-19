use image::{GenericImageView, ImageReader};

use crate::get_parameter;
use crate::log::get_conf_log_path;
use crate::parse::Idata;

pub fn get_image(s: String) -> Option<Vec<u8>> {
    println!("{}", s);
    match dirs::home_dir() {
        Some(mut path) => {
            path.push(".control_scrapper");
            path.push("control_scrapper");
            path.set_extension("conf");

            let log_path = get_conf_log_path();

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

pub fn store_image(image_bytes: &Vec<u8>) -> bool {
    let path = get_conf_image_path();
    if !path.is_empty() {
        std::fs::write(path, image_bytes).is_ok()
    } else {
        false
    }
}

pub fn check_image(image_bytes: &Vec<u8>) -> bool {
    match check_image_inner(image_bytes) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Error checking image dimensions");
            false
        }
    }
}

pub fn check_image_inner(image_bytes: &Vec<u8>) -> Result<bool, anyhow::Error> {
    let img = ImageReader::new(std::io::Cursor::new(image_bytes.as_slice()))
        .with_guessed_format()?
        .decode()?;
    let dim = img.dimensions();
    let idata = Idata {
        w: dim.0 as usize,
        h: dim.1 as usize,
        path: "".to_string(),
    };
    Ok(idata.has_right_dimensions_slow())
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
