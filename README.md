# Desktop Copy
This is a simple program to copy Bing Wallpapers from their source directory to a destination directory.

![Screenshot 2024-10-20 210857](https://github.com/user-attachments/assets/96aca2b3-441d-4d4b-804f-0cbe23df0b3e)

## Usage
_rust must be installed https://www.rust-lang.org/tools/install_
1. Clone the Repo
```sh 
git clone https://github.com/jr0dsgarage/desktop_copy.git
```
2. in `/src/main.rs`, near the top:
   - Change the `src` directory string to your own source directory.
   - Change the `dest` directory string to your desired destination
3. In a terminal, in the main desktop_copy directory, type
```sh
cargo run
```
4. To build a release copy of the program, type 
```sh
cargo build --release
```
The built program will be located in `/target/release/`

## Dependencies

This project uses the following crate dependencies:

- [`colored`](https://crates.io/crates/colored) for colored terminal output
