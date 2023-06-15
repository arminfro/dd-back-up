use chrono::Local;

pub fn current_date() -> String {
    let current_date = Local::now();
    current_date.format("%Y-%m-%d").to_string()
}
