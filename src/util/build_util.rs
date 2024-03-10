use chrono::{DateTime, Utc};

pub struct BuildUtil;

impl BuildUtil {
  pub fn gen_build_number(build_number: i64) -> i64 {
    let now: DateTime<Utc> = Utc::now();
    let mut build_number = build_number.to_string();
    if (build_number.len() != 8) {
      return format!("{}01", now.format("%y%m%d")).parse::<i64>().unwrap();
    } else {
      let date_ole = build_number[0..6].to_string();
      let date_new = now.format("%y%m%d").to_string();
      return if (date_ole != date_new) {
        format!("{}01", date_new).parse::<i64>().unwrap()
      } else {
        let build = build_number[6..8].parse::<i64>().unwrap() + 1;
        format!("{}{:02}", date_new, build).parse::<i64>().unwrap()
      }
    }
  }
}
