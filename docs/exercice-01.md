# Practise 01 Guessing Game

https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

Create a guessing generating a random integer between 1 and 100. The game will prompt the player to enter a guess and it indicates the player if is to low or too high. If the guess is correct, the game will send a congratulations message

- We need `io` library (included by [default](https://doc.rust-lang.org/std/prelude/index.html))

- Storing values with variables

  - `let mut guess = String::new();` -> creating a new mutable variable
  - `let apples = 5;`
  - This line creates a new variable and binds it to the value 5.
  - In Rust, **variables are inmutable by default**. This means that once we give a value to a variable, the value won´t change.
  - `String::new()` -> Creates a new instance of a String. It is a type provided by the Standard library

- User Input

  - `read_line(&mut guess)` -> we are passing a mutable variable as an argument to the read_line method
  - `expect(msg)` -> **Warning** If you don´t call expect, the program will compile, but you ´ll get a warning message

- Printing Values with println! Placeholders

```rust
let x = 5;
let y = 10;
println!("x = {} and y = {}", x, y);
//or
println!("x = {x} and y = {y}");
```

- Generating a Secret Number with a crate

  - _Information_ A crate is a collection of Rust source code files.
  - Add the crate `rand` to the cargo.toml file `rand = "0.8.3"`

- Comparing the guess to the secret number
  - **_ Important _**
  - Rust provides pattern matching with `match` keyword (like a switch)
  - We are using the module [std::cmp](https://doc.rust-lang.org/std/cmp/index.html), and the tool `Ordering`
    - It is a utility for comparing and ordering values
    - Returns an enum
    - ```
        pub enum Ordering {
          Less,
          Equal,
          Greater,
        }
      ```

```rust
match guess.cmp(&secret_number) {
  Ordering::Less => println!("Too low!"),
  Ordering::Equal => {
      println!("You win! 🏆");
      break;
  }
  Ordering::Greater => println!("Too high!"),
}
```

We are saying to Rust that match guess value and compare with one of the ordering results
