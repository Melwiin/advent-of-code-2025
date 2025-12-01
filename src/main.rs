#![allow(warnings)] // Surpress warning... blah blah

mod days;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <day>");
        return;
    }

    let day = &args[1];

    match day.as_str() {
        "1" | "01"  => days::day01::run(),
        "2" | "02"  => days::day02::run(),
        "3" | "03"  => days::day03::run(),
        "4" | "04"  => days::day04::run(),
        "5" | "05"  => days::day05::run(),
        "6" | "06"  => days::day06::run(),
        "7" | "07"  => days::day07::run(),
        "8" | "08"  => days::day08::run(),
        "9" | "09"  => days::day09::run(),
        "10"        => days::day10::run(),
        "11"        => days::day11::run(),
        "12"        => days::day12::run(),
        _ => eprintln!("Unknown day {}", day),
    }
}
