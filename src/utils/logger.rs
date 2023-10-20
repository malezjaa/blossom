use std::collections::HashMap;
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

pub fn print_packages(title: &str, packages: &HashMap<String, String>) {
    if !packages.is_empty() {
        println!("\n{}", title.cyan());
        for (name, version) in packages {
            println!("{} {} {}", "+".bold().cyan(), name.bold(), version.bold().truecolor(106, 106, 106));
        }
    }
}