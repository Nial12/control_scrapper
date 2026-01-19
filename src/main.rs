use config::get_config;
use grab::grabhtml;
use image::{get_image, store_image};
use parse::{clean_idata_vec, parsehtml};
use rand::seq::SliceRandom;

use crate::{
    grab::{actualise_txt_list, grabtxt},
    image::check_image,
    parse::{parsetxt, remove_from_list},
};

mod config;
mod grab;
mod image;
mod log;
mod parse;

thread_local! {
    pub static CONFIG: std::collections::HashMap<String, String> = get_config();
}

fn main() {
    let from_txt_list = get_from_txt_list();
    if !from_txt_list {
        let tmp = grabhtml();
        let mut v = parsehtml(tmp);
        clean_idata_vec(&mut v);
        let mut rng = rand::thread_rng();
        while v.is_empty() {
            let tmp = grabhtml();
            v = parsehtml(tmp);
        }
        let idata = v.choose(&mut rng).unwrap();

        if let Some(image_byte) = get_image(idata.path.clone()) {
            store_image(&image_byte);
        }
    } else {
        let mut tmp = grabtxt();
        let mut v = parsetxt(&tmp);
        let mut rng = rand::thread_rng();
        let mut idata = v.choose(&mut rng).unwrap();

        if let Some(mut image_bytes) = get_image(idata.path.clone()) {
            while !check_image(&image_bytes) {
                remove_from_list(&mut tmp, &idata.path);
                v = parsetxt(&tmp);
                idata = v.choose(&mut rng).unwrap();
                image_bytes = get_image(idata.path.clone()).unwrap()
            }
            store_image(&image_bytes);
            actualise_txt_list(&tmp);
        }
    }
}

get_parameter!(from_txt_list, bool, true, parse);
