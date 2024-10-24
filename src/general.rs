use colored::*;
use image::{Rgb, RgbImage};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use qrcode::QrCode;
use qrcode::render::unicode;

use std::{
    fs,
    io::{stdout, Write},
    process::Command,
    thread::sleep,
    time::Duration,
};

pub fn welcome() {
    let text = r#"
    __________________  ___  ___            _ _             
    | ___ |  _  | ___ \ |  \/  |           (_| |            
    | |_/ | | | | |_/ / | .  . | ___  _ __  _| |_ ___  _ __ 
    |    /| | | |  __/  | |\/| |/ _ \| '_ \| | __/ _ \| '__|
    | |\ \| |/ /| |     | |  | | (_) | | | | | || (_) | |
    \_| \_|___/ \_|     \_|  |_/\___/|_| |_|_|\__\___/|_|    
   "#;
    let lines: Vec<&str> = text.split('\n').collect();

    for (i, line) in lines.iter().enumerate() {
        let color = match i % 6 {
            0 => Color::Red,
            1 => Color::Green,
            2 => Color::Yellow,
            3 => Color::Blue,
            4 => Color::Magenta,
            5 => Color::Cyan,
            _ => Color::White, // Default color
        };
        println!("{}", line.color(color));
        stdout().flush().unwrap();
        sleep(Duration::from_millis(30));
    }
}

pub fn color_print(sentence: &str, color: &str) {
    let reset = "\x1b[0m";
    let color_code = match color {
        "red" => "\x1b[0;31m",
        "green" => "\x1b[0;32m",
        "yellow" => "\x1b[0;33m",
        "cyan" => "\x1b[36m",
        _ => reset,
    };
    //println!();
    for c in sentence.chars() {
        print!("{}{}", color_code, c);
        stdout().flush().unwrap();
        sleep(Duration::from_millis(20));
    }
    print!("{}\n", reset);
}

pub fn run_command(command: &str) -> Result<String, std::io::Error> {
    let output = Command::new("cmd").arg("/C").arg(command).output()?;
    //let output = cmd.output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }
}

pub fn get_user_id() -> Result<String, ()> {
    let result = run_command("whoami").unwrap_or_else(|err| {
        eprintln!("Error while getting the user_id: {}", err);
        "".to_string()
    });
    let user_id = result.split('\\').last().unwrap_or(&result).trim();
    if !user_id.is_empty() {
        Ok(user_id.to_string())
    } else {
        Err(())
    }
}

pub fn get_user_fullname() -> Option<(String, String)> {
    let user_id = get_user_id().unwrap();
    let cmd = format!("net user /DOMAIN {}", user_id);
    let result: String = run_command(&cmd).unwrap_or_else(|err| {
        eprintln!("Error while getting user's full name! \n{}", err);
        "".to_string()
    });
    if user_id == "g-gtvt01" {
        let fullname: String = run_command("hostname").unwrap_or_else(|err| {
            eprintln!("Error while getting hostname for {}! \n{}", user_id, err);
            "".to_string()
        });
        return Some((fullname.trim().to_string(), "IFOS MACHINE".to_string()));
    }
    for line in result.lines() {
        if line.contains(" Name") {
            let name_team = line.split(" Name").nth(1).unwrap().trim_start();
            let name = name_team.split('(').next().unwrap().trim_end();
            let team_str = name_team.split('(').nth(1).unwrap();
            let team = &team_str[..team_str.len() - 1];
            return Some((name.to_string(), team.to_string()));
        }
    }
    None
}

pub fn entropy_gen(file_path: &str) {
    let size = 30;
    let mut rng = StdRng::from_entropy();
    // Create a new RGB image buffer
    let mut img = RgbImage::new(size, size);
    // Generate random colors and set pixels
    for y in 0..size {
        for x in 0..size {
            let r: u8 = rng.gen();
            let g: u8 = rng.gen();
            let b: u8 = rng.gen();

            let pixel = Rgb([r, g, b]);
            img.put_pixel(x, y, pixel);
        }
    }
    img.save(file_path).unwrap();
}

pub fn create_folder(foldername: &str) {
    if !std::path::Path::new(foldername).exists() {
        if let Err(err) = fs::create_dir(foldername) {
            eprintln!("Error createing {foldername}:\n{err}");
        }
    }
}


pub fn mailto_qr_gen(){
    let recipient = "dariush.shiri@gmail.com";
    let subject = "RDP_Monitor Related";
    let body = "Hello there!\n\n";
    let mailto_uri = format!("mailto:{}?subject={}&body={}", recipient, subject, body);
    let code = QrCode::new(mailto_uri).unwrap();
    let unicode = code.render::<unicode::Dense1x2>()
        .quiet_zone(false)
        .build();
    println!("{}", unicode);
}
