use regex::Regex;
use scraper::{Html, Selector};

#[derive(Default, Debug)]
pub struct Idata {
    pub w: usize,
    pub h: usize,
    pub path: String,
}

pub fn parsehtml(s: String) -> Vec<Idata> {
    let mut v: Vec<Idata> = vec![];
    let mut c: usize = 0;
    let r_width = Regex::new(r"width=.(?<width>\d+).").unwrap();

    let r_height = Regex::new(r"height=.(?<height>\d+).").unwrap();

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
    while i < v.len() {
        if ((v[i].w * 9) / 16 == v[i].h) && ((v[i].w * 9) % 16 == 0) {
            i += 1;
        } else {
            v.remove(i);
        }
    }
}
