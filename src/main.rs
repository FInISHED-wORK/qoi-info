use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::process::exit;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Usage: ./qoi-info <input>.qoi");
        exit(1);
    }

    let input_file = match args.get(1) {
        Some(x) => x,
        None => {
            println!("ERROR: Input file not specified!");
            exit(1);
        },
    };

    let mut file = match File::open(input_file) {
        Ok(value) => value,
        Err(value) => {
            println!("ERROR {:?}", value.to_string());
            exit(1);
        }
    };

    let mut ptr = 0;

    let magic: [u8; 4] = read_section(&mut ptr, &mut file);
    if magic != [113, 111, 105, 102] {
        println!("Error: Given file isn't a valid QOI image.");
        exit(1);
    }

    let width_buf: [u8; 4] = read_section(&mut ptr, &mut file);
    let height_buf: [u8; 4] = read_section(&mut ptr, &mut file);
    let channels: [u8; 1] = read_section(&mut ptr, &mut file);
    let colorspace: [u8; 1] = read_section(&mut ptr, &mut file);
    
    let channel = match channels[0] {
        3 => "RBG",
        4 => "RGBA",
        _ => {
            println!(
                "ERROR:Unknown channel format with {:?} components",
                channels[0]
            );
            exit(-1);
        }
    };

    let cs = match colorspace[0] {
        0 => "sRGB with linear alpha",
        1 => "all channels linear",
        _ => {
            println!("ERROR: Unknown colorspace format {:?}", colorspace[0]);
            exit(-1);
        }
    };

    println!(
        "File {}:\n\tSize: {}x{}\n\tChannels: {}\n\tColorspace: {}",
        input_file,
        read_u32_from_u8_buffer(width_buf),
        read_u32_from_u8_buffer(height_buf),
        channel,
        cs
    );
}

fn read_u32_from_u8_buffer(buffer: [u8; 4]) -> u32 {
    return u32::from(buffer[0]) << 24
        | u32::from(buffer[1]) << 16
        | u32::from(buffer[2]) << 8
        | u32::from(buffer[3]);
}

fn read_section<const S: usize>(ptr: &mut u64, file: &mut File) -> [u8; S] {
    let mut buf: [u8; S] = [0; S];
    _ = match file.seek(SeekFrom::Start(*ptr)) {
        Ok(value) => value,
        Err(e) => {
            println!("Error: {}", e);
            exit(1);
        },
    };
    _ = match file.read(&mut buf) {
        Ok(value) => value,
        Err(e) => {
            println!("Error: {}", e);
            exit(1);
        },
    };
    *ptr += S as u64;
    return buf;
}
