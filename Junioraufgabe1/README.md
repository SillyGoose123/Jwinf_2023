# Juioraufgabe1

## How to run the code?

- Run the exe file in the folder `Junioraufgabe1/AusführbaresProgramm/wundertuete.exe`
- Use `cargo run` in the folder `Junioraufgabe1/` (Rust need to be installed)
- Use `cargo build --release` in the folder `Junioraufgabe1/` the executable will be in the folder `Junioraufgabe1/target/release` (Rust need to be installed)

## How to use the program?

- You can pass the path to the input file as an argument to the program to use them as input
- If you don't pass an argument the program will ask you for input

## How does the program work?

1. The program will get input from the user or from the file in the arguments
2. The algorithm starts with the first game type and distributes it until there is no game from the type left, then the next game type will be distributed and so on until there is no game type left
3. The result from the algorithm is given to the formatter which will format it so that it is readable for the user
4. The formatted result will be printed out to the user
