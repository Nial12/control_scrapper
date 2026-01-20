use std::{fs::File, io::Read, vec};

use chrono::{Datelike, NaiveDate};
use rand::seq::SliceRandom;
use reqwest::blocking::Client;

use crate::{get_conf_from_txt_list, get_parameter};

pub fn grabhtml() -> String {
    let mut s: String = get_conf_base_url();
    for c in rndmonth().chars() {
        s.push(c);
    }
    s.push('/');
    // let s = "https://505games.com/control-faden-friday-december-2024/".to_owned();
    let url = url::Url::parse(&s[..]).unwrap();
    let client: Client = reqwest::blocking::Client::new();
    let mut body = client.get(url.clone());
    body = body.timeout(std::time::Duration::from_secs(300));
    let mut req = body.send().unwrap();
    while !req.status().is_success() {
        // println!("Failed Request at {}", req.url());
        let mut sn: String = get_conf_base_url();
        for c in rndmonth().chars() {
            sn.push(c);
        }
        sn.push('/');
        let urln = url::Url::parse(&sn[..]).unwrap();
        body = client.get(urln.clone());
        body = body.timeout(std::time::Duration::from_secs(300));
        req = body.send().unwrap();
    }
    req.text().unwrap() /* .escape_unicode().to_string() */
}

pub fn rndmonth() -> String {
    let today: chrono::NaiveDate = chrono::offset::Utc::now()
        .date_naive()
        .checked_sub_months(chrono::Months::new(1))
        .unwrap()
        .with_day(1)
        .unwrap();
    let mut v: Vec<chrono::NaiveDate> = vec![];
    let begin: chrono::NaiveDate = NaiveDate::from_ymd_opt(2023, 2, 1).unwrap();
    let mut i: u32 = 0;
    while begin.checked_add_months(chrono::Months::new(i)).unwrap() <= today {
        v.push(begin.checked_add_months(chrono::Months::new(i)).unwrap());
        i += 1;
    }
    let mut rng = rand::thread_rng();
    v.choose(&mut rng)
        .unwrap()
        .format(&get_conf_date_format()[..])
        .to_string()
}

pub fn grabtxt() -> String {
    match grabtxt_inner() {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Error while obtaining image list from txt file");
            "".to_owned()
        }
    }
}

pub fn grabtxt_inner() -> Result<String, anyhow::Error> {
    let txt_path = get_conf_image_list_txt_path();
    let mut image_list = File::open(txt_path)?;
    let mut file_byte = Vec::new();
    image_list.read_to_end(&mut file_byte)?;
    Ok(String::from_utf8(file_byte.to_vec())?)
}

pub fn actualise_txt_list(s: &String) -> bool {
    if !get_conf_from_txt_list() {
        return true;
    }
    let path = get_conf_image_list_txt_path();
    if !path.is_empty() {
        std::fs::write(path, s).is_ok()
    } else {
        false
    }
}

// See format rules here : https://docs.rs/chrono/latest/chrono/format/strftime/index.html
get_parameter!(date_format, String, "%B-%Y", to_owned);
get_parameter!(
    base_url,
    String,
    "https://505games.com/control-faden-friday-",
    to_owned
);
get_parameter!(
    image_list_txt_path,
    String,
    {
        match dirs::home_dir() {
            Some(mut path) => {
                path.push(".control_scrapper");
                path.push("image_list");
                path.set_extension("txt");
                path.display().to_string()
            }

            None => {
                eprintln!("Impossible to get your home dir!");
                "image_list.txt".to_string()
            }
        }
    },
    to_owned
);
