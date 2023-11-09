use std::panic;
use std::process::exit;
use std::io::Write;

//the struct for the input
struct Input {
    bag_count: i64,
    game_types: Vec<i64>,
}

fn main() {
    //custom error handling
    panic::set_hook(Box::new(|info| {
        //print the error
        eprintln!("Error: {}", &info.to_string().split("'").collect::<Vec<_>>()[1]);

        // kill the program with exit code 1
        exit(1);
    }));

    let user_input: Input;

    //check for file input
    if std::env::args().len() > 2 {
        println!("Error: Incorrect usage. Input files like this <input.txt>.");
        exit(1);
    } else if std::env::args().len() == 2 {
        //get the input from the file as vector of strings
        let file_content = match std::fs::read_to_string(std::env::args().collect::<Vec<_>>()[1].clone()) {
            Ok(f) =>f,
            Err(_err) => panic!("File not found or wrong encoding."),
        };

        //split the file content into a vector of strings
        let file_content = file_content.trim().split("\n").collect::<Vec<_>>();

        //check if the file is formatted right
        if file_content.len() < 3 {
            panic!("Given file is formatted wrong.");
        }

        if int_from_str(&file_content[0]) <= 0  || file_content.contains(&"0") {
            panic!("File input makes no sense.");
        }

        //get the game_types from the file
        let mut game_types: Vec<i64> = Vec::new();
        for i in 2..file_content.len() {
            game_types.push(int_from_str(&file_content[i]));
        }

        //insert into an struct and return the struct
        user_input = Input {
            bag_count: int_from_str(&file_content[0]),
            game_types,
        }

    } else {
        //get input from user
        user_input = get_input_from_user();
    }

    //calculate the result
    let result = calculate_the_bags(user_input);

    //format result
    let formatted_result = format_result(result);

    //tell the user the result
    println!("Die Lösung lautet:\n{}", formatted_result);
}

fn get_input_from_user() -> Input {
    //create a vector for the game_types
    let mut game_types: Vec<i64> = Vec::new();

    //get the count for every game_types
    for i in 0..read_int("Bitte schreiben sie die Anzahl an game_types."){
        game_types.push(read_int(format!("Bitte schreiben sie die Anzahl an Spielen für die {} Spielsorte.", i + 1).as_str()));
    }

    // insert into an struct and return the struct
    Input {
        bag_count: read_int("Bitte schreiben sie die Anzahl an Wundertüten."),
        game_types,
    }
}

fn calculate_the_bags(user_input: Input) -> Vec<Vec<i64>> {
    //create the result vector
    let mut result = Vec::new();
    let mut game_types = user_input.game_types.clone();
    let bag_count = user_input.bag_count;

    //init the result vector with null values
    let mut empty_games = Vec::new();
    for _i in 0..game_types.len() {
        empty_games.push(0);
    }

    //fill the result vector with empty games
    for _i in 0..bag_count {
        result.push(empty_games.clone());
    }

    //fill the result vector with games
    let mut current_bag = 0;
    loop {
        let current_game_type = give_next_game(&mut game_types);

        //check if there are no more left
        if current_game_type == -1 {
            break;
        }

        //add the game to the current bags
        result[current_bag][current_game_type as usize] += 1;

        //next bag
        current_bag += 1;

        //check if the current_bag is the last one
        if current_bag == bag_count as usize {
            current_bag = 0;
        }
    }

    //return the result
    result
}

fn give_next_game(game_types: &mut Vec<i64>) -> i64 {
    //check all game_types for games
    for i in 0..game_types.len() {
        // if there are games left take one and return the game_types
        if game_types[i] > 0 {
            //remove one game from the game_types
            game_types[i] -= 1;

            //return the game_types which has a game left
            return i as i64;
        }
    }

    //if there are no games left return -1
    -1
}

fn format_result(bags: Vec<Vec<i64>>) -> String {
    //create result string
    let mut result = String::new();

    //fill the result string with content
    for i in 0..bags.len() {
        //add the bag with number to the result
        result+= format!("Wundertuete {}:\n", i + 1).as_str();

        //add the game_types from the bag with the number i to the result
        for j in 0..bags[i].len() {
            result += format!(" {}x Spiel {},\n", bags[i][j], j + 1).as_str();
        }

        //remove the last comma and add a new line
        result = result.to_string().trim_end_matches("\n").to_string().trim_end_matches(",").to_string();
        result += "\n\n";
    }

    //return the result without the the last empty lines and remove the last comma
    result.trim_end_matches("\n").to_string()
}

// a function to read an int from the user
pub fn read_int(message: &str) -> i64 {
    println!("{} ", message);
    loop {
        // for ux (user experience)
        print!("> ");

        //clear the tests buffer so the print combined with the input above works
        std::io::stdout().flush().unwrap();

        //create a string for the input
        let mut input = String::new();

        //read the input
        std::io::stdin().read_line(&mut input).expect("Failed to read from stdin");

        //check if it is parseable into an i64 if not ask again
        match input.trim().parse::<i64>() {
            Ok(ok) => return ok,
            Err(_e) => print!("Bitte gib eine Dezimalzahl ein."),
        }
    }
}

pub fn int_from_str(input:&str) -> i64 {
    match input.trim().parse::<i64>() {
        Ok(ok) => return ok,
        Err(_e) => panic!("File is broken couldn't parse {}.", input),
    }
}