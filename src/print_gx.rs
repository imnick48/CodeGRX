use std::{io::{stdout, Write}, thread, time::Duration};

use colored::*;
use crossterm::{cursor, terminal, ExecutableCommand};
use figlet_rs::FIGfont;

fn clear_screen() {
    let mut stdout = stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();
}

fn typing_effect(text: &str, color: Color) {
    for ch in text.chars() {
        print!("{}", ch.to_string().color(color));
        stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(35));
    }
    println!();
}

pub fn car_gx() {
    clear_screen();

    // ASCII Art Banner
    let font = FIGfont::standard().unwrap();
    let banner = font.convert("CodeGRX").unwrap();
    println!("{}", banner.to_string().bright_cyan());

    // Subtitle
    println!("{}", "The Intelligent CLI Agent for Devs".bold().bright_magenta());
    println!("{}", "----------------------------------------------------".dimmed());

    // Typing Intro
    typing_effect("🔧 Booting up modules...", Color::Yellow);
    typing_effect("🧠 Connecting to neural code core...", Color::Blue);
    typing_effect("✅ Initialization complete.", Color::Green);
    println!();
}
