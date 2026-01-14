use config::get_config;
use grab::grabhtml;
use image::{get_image, store_image};
use parse::{clean_idata_vec, parsehtml};
use rand::seq::SliceRandom;

mod config;
mod grab;
mod image;
mod log;
mod parse;

thread_local! {
    pub static CONFIG: std::collections::HashMap<String, String> = get_config();
}

fn main() {
    let tmp = grabhtml();
    let mut v = parsehtml(tmp);
    clean_idata_vec(&mut v);
    let mut rng = rand::thread_rng();
    while v.is_empty() {
        let tmp = grabhtml();
        v = parsehtml(tmp);
    }
    let idata = v.choose(&mut rng).unwrap();

    if let Some(image) = get_image(idata.path.clone()) {
        store_image(image);
    }
}
