use chrono::{Datelike, NaiveDate};
use rand::seq::SliceRandom;
use reqwest::blocking::Client;

pub fn grabhtml() -> String {
    let mut s: String = "https://controlgame.com/faden-friday-".to_string();
    for c in rndmonth().chars() {
        s.push(c);
    }
    s.push('/');

    let url = url::Url::parse(&s[..]).unwrap();
    let client: Client = reqwest::blocking::Client::new();
    let mut body = client.get(url.clone());
    body = body.timeout(std::time::Duration::from_secs(300));
    let mut req = body.send().unwrap();
    while !req.status().is_success() {
        let mut sn: String = "https://controlgame.com/faden-friday-".to_string();
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
    v.choose(&mut rng).unwrap().format("%B-%C%y").to_string()
}
