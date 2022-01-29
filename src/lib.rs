use chrono::{Datelike, Local};
use std::env;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate() -> u32 {
    let local_date = Local::now();
    let salt_env = env!("SALT");
    let factor_env = env!("FACTOR");
    let salt = salt_env.parse::<u32>().unwrap();
    let factor = factor_env.parse::<u32>().unwrap();
    let yr = local_date.year() as u32;
    let mo = local_date.month() as u32;
    let dy = local_date.day() as u32;

    (yr + mo + dy + salt) * factor
}
