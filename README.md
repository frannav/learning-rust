# Learning Rust 🦀

- Docu Links: https://doc.rust-lang.org/book/title-page.html

## Compile

```
rustc ./src/main.rs
```

That command compile main.rs file into binary file.

## Executing binary files

```
main
```

### Troubleshooting

- Permission denied executing binary file: `chmod +x ./ && ./main`

## Cargo

Compiling with `rustc` is fine for simple programs, but as your project grows, you’ll want to manage all the options and make it easy to share your code.
Cargo is Rust build´s system and package manager. Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries.

- `cargo new new-project` -> Creates a new cargo project
- `cargo init` -> initializes cargo on the current dir
- `cargo build` -> similar to `rustc` (rust compiler) but it creates binary on ./target/debug/new-project directory. Cargo put this file on this directory because **_default build is a debug build_**
- `cargo build --release` -> Compiling with optimizations, this command creates an executable in target/release that runs much flaster
- `cargo run` -> Compile the code and run the resulting binary
- `cargo check` -> Checks if your code compiles but without producing an executable

## Practise 01 Guessing Game

Create a guessing generating a random integer between 1 and 100. The game will prompt the player to enter a guess and it indicates the player if is to low or too high. If the guess is correct, the game will send a congratulations message
