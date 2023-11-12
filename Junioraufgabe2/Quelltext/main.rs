#![feature(ascii_char)]

use std::io::{Write};
use std::panic;
use std::path::Path;
use std::process::exit;
use image::{GenericImageView};

fn main() {
    //custom error handling
    panic::set_hook(Box::new(|info| {
        //print the error
        eprintln!("Error: {}", &info.to_string().split("'").collect::<Vec<_>>()[1]);

        // kill the program with exit code 1
        exit(1);
    }));

    //get the input from the file as vector of bytes
    let file_path;

    //check for file input
    if std::env::args().len() > 2 {
        println!("Error: Incorrect usage. Input files like this <input.txt>.");
        exit(1);
    } else if std::env::args().len() == 2 {
        file_path = std::env::args().collect::<Vec<_>>()[1].clone();
    } else {
        //get input from user
        file_path = get_input_from_user();
    }

    println!("File path: {}:", file_path);

    //read the file
    let img = match image::open(&Path::new(&file_path))  {
        Ok(ok) => ok,
        Err(_e) => panic!("Error: Could not open file."),
    };


    //get the width and height of the image
    let img_width = img.dimensions().0;
    let img_height = img.dimensions().1;

    //vars for the loop
    let mut result: Vec<u8> = Vec::new();
    let mut pos = vec![0,0];

    loop {
        let current_pixel: Vec<u8> = img.get_pixel(pos[0], pos[1]).0.to_vec();

        //get the ascii value of the current pixels R value
        result.push(current_pixel[0]);

        //find out were to go next
        pos[0] = (current_pixel[1] as u32 + pos[0]) % img_width;
        pos[1] = (current_pixel[2] as u32 + pos[1]) % img_height;

        //check for end
        if current_pixel[1] == 0 && current_pixel[2] == 0 {
            break;
        }
    }

    println!("Result: {}", get_ascii_string_from_u8_vec(result));
}

fn get_input_from_user() -> String {
    loop {
        // for ux (user experience)
        print!("Enter the Path to the File you want to decode> ");

        //clear the tests buffer so the print combined with the input above works
        std::io::stdout().flush().unwrap();

        //create a string for the input
        let mut input = String::new();

        //read the input
        std::io::stdin().read_line(&mut input).expect("Failed to read from stdin.");

        //check if it is parseable into an i64 if not ask again
        match input.trim().parse::<String>() {
            Ok(ok) => return ok,
            Err(_e) => print!("Bitte gib einen Path ein."),
        }
    }
}

fn get_ascii_string_from_u8_vec(input: Vec<u8>) -> String {
    let mut result = String::new();

    //iterate over the input
    for i in 0..input.len() {
        //check if the current byte is a valid ascii char
        let current_letter = input[i].as_ascii();
        if !current_letter.is_none() {
            result.push(current_letter.unwrap().as_char());
        } else {
            //check for special german characters
            result.push_str(match input[i] {
                195 => match input[i + 1] {
                    132 => "Ä",
                    164 => "ä",
                    150 => "Ö",
                    182 => "ö",
                    156 => "Ü",
                    188 => "ü",
                    159 => "ß",
                    _ => "",
                }
                252 => "ü",
                228 => "ä",
                246 => "ö",
                223 => "ß",
                214 => "Ö",
                220 => "Ü",
                196 => "Ä",
                _ => "",
            });
        }
    }

    result
}