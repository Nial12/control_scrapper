use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn get_config() -> HashMap<String, String> {
    let mut conf: HashMap<String, String> = HashMap::new();

    match dirs::home_dir() {
        Some(mut path) => {
            path.push(".control_scrapper");
            path.push("control_scrapper_d");
            path.set_extension("conf");
            if let Ok(lines) = read_lines(path) {
                for line in lines.map_while(Result::ok) {
                    if let Some(trimed) = line.split("#").next() {
                        let splited = trimed.split("=");
                        let vec = splited.collect::<Vec<_>>();
                        if vec.len() == 2 && !vec[0].trim().is_empty() && !vec[1].trim().is_empty()
                        {
                            conf.insert(vec[0].trim().to_string(), vec[1].trim().to_string());
                        }
                    };
                }
            }
        }
        None => {
            eprintln!("Impossible to get your home dir!");
        }
    }
    conf
}
// This comes from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[macro_export]
macro_rules! get_parameter {
    ($config_key: ident, $type:ty, $default:expr, parse) => {
        paste::paste! {
                #[inline]
                pub fn [<get_ $config_key>]() -> $type {
                    let default_value = $default;
                    $crate::CONFIG.with(|config| -> $type {
                        if let Some(temp) = config.get(stringify!($config_key)) {
                            temp.to_lowercase()
                                .parse::<$type>()
                                .unwrap_or(default_value)
                        } else {
        default_value
                        }
                    })
                }
            }
    };

    ($config_key:ident, $type:ty, $default:expr, to_owned) => {
        paste::paste! {
            #[inline]
            pub fn [<get_ $config_key>]() -> $type {
                let default_value = $default;
                $crate::CONFIG.with(|config| -> $type {
                    if let Some(temp) = config.get(stringify!($config_key)) {
                        temp.to_owned()
                    } else {
                        default_value.to_owned()
                    }
                })
            }
        }
    };
}
