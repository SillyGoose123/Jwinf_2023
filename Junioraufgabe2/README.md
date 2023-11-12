# Juioraufgabe2

## How to run the code?

- Run the exe file in the folder `Junioraufgabe2/AusführbaresProgramm/st_egano.exe`
- Use `cargo run` in the folder `Junioraufgabe2/` (Rust need to be installed)
- Use `cargo build --release` in the folder `Junioraufgabe2/` the executable will be in the folder `Junioraufgabe1/target/release` (Rust need to be installed)

## How to use the program?

- You can pass the path to the PNG-File as an argument to the program to use it as input
- If you don't pass an argument the program will ask you for input

## How does the program work?

1. The program will get input from the user or from the file in the arguments
2. Read the file
3. Start from Pixel (0,0) save R to the ASCII-Code-Array
4. Go to the next Pixel that is Calculated by the following terms `(x + G) % img_width` and `(y + G) % img_height`
5. Repeat step 4 until you reach the Pixel with G & B is 0
6. Convert the ASCII-Code-Array to a String
7. Print the String to the user