use regex::Regex;
use scraper::{Html, Selector};

use crate::get_parameter;

#[derive(Default, Debug)]
pub struct Idata {
    pub w: usize,
    pub h: usize,
    pub path: String,
}

pub fn parsehtml(s: String) -> Vec<Idata> {
    let mut v: Vec<Idata> = vec![];
    let mut c: usize = 0;
    let r_width = Regex::new(&get_width_regex()).unwrap();

    let r_height = Regex::new(&get_height_regex()).unwrap();

    let r_path = Regex::new(r"src=.(?<path>[a-zA-Z:\/\.\-0-9@\_]+).").unwrap();

    let fragment = Html::parse_fragment(&s);
    let selector = Selector::parse("img").unwrap();

    for element in fragment.select(&selector) {
        if let Some(idata) = textcheck(
            element.html(),
            &mut c,
            r_width.clone(),
            r_height.clone(),
            r_path.clone(),
        ) {
            v.push(idata);
        }
    }
    v
}

pub fn textcheck(
    s: String,
    c: &mut usize,
    r_width: Regex,
    r_height: Regex,
    r_path: Regex,
) -> Option<Idata> {
    let mut idata = Idata {
        w: 0,
        h: 0,
        path: "".to_string(),
    };
    if let Some(cap) = r_width.captures(s.as_str()) {
        idata.w = cap
            .name("width")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
    }
    if let Some(cap) = r_height.captures(s.as_str()) {
        idata.h = cap
            .name("height")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
    }
    if let Some(cap) = r_path.captures(s.as_str()) {
        idata.path = cap.name("path").unwrap().as_str().to_string();
    }
    if idata.w == 0 || idata.h == 0 || idata.path.is_empty() {
        None
    } else if *c <= 1 {
        *c += 1;
        None
    } else {
        Some(idata)
    }
}

pub fn clean_idata_vec(v: &mut Vec<Idata>) {
    let mut i: usize = 0;
    let target_width = get_target_width();
    let target_height = get_target_height();
    let at_least_as_large = get_at_least_as_large();
    while i < v.len() {
        if v[i].w * target_height == v[i].h * target_width {
            if !at_least_as_large || (target_width <= v[i].w && target_height <= v[i].h) {
                i += 1;
            } else {
                v.remove(i);
            }
        } else {
            v.remove(i);
        }
    }
}

get_parameter!(target_height, usize, 1080, parse);
get_parameter!(target_width, usize, 1920, parse);
get_parameter!(at_least_as_large, bool, true, parse);
get_parameter!(height_regex, String, r"height=.(?<height>\d+).", to_owned);
get_parameter!(width_regex, String, r"width=.(?<width>\d+).", to_owned);
