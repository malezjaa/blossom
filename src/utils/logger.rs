use chrono::Local;
use colored::*;

pub fn info(message: &str) {
    let message = format!("{} {}", " INFO ".black().bold().on_blue(), message);

    println!("{}", message);
}

pub fn error(message: &str) {
    let message = format!("{} {}", " ERROR ".black().bold().on_red(), message);

    println!("{}", message);
}

pub fn success(message: &str) {
    let message = format!("{} {}", " SUCCESS ".black().bold().on_green(), message);

    println!("{}", message);
}
