use std::io::{Write};
use std::panic;
use std::path::Path;
use std::process::exit;
use image::{GenericImageView, Rgb, Rgba};

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

    println!("File path: {}", file_path);

    //read the file
    let mut img = match image::open(&Path::new(&file_path))  {
        Ok(ok) => ok,
        Err(_e) => panic!("Error: Could not open file."),
    };

    let img_width = img.dimensions().0;
    let img_height = img.dimensions().1;


    //du keke mach for loop und dann machst du das mit den pixeln

    let pixels: Vec<_> = img.get_pixel(0, 0).0.to_vec();







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