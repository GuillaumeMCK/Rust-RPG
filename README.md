# RustPG 🦀⚔️

---

> A project to learn Rust. It's using the [GGez](https://ggez.rs/) game engine.
> The game logic is based on a OOP subject given during my studies at the ASTON School of Computer Science in France.

> **Note**: You can find the subject [here](). (only in french)

<div align="center"><H3>🚧 - WIP - 🚧</H3></div>

## Table of Contents

- [RustPG ⚔️](#rustpg-)
    - [Table of Contents](#table-of-contents)
    - [Rust](#rust)
        - [What is Rust?](#what-is-rust)
        - [How Rust OOP works?](#how-rust-oop-works)
    - [Diagram](#diagram)
    - [Installation](#installation)
        - [Libraries](#libraries)
        - [Build](#build)
    - [Credits](#credits)
    - [License](#license)

## Rust

### What is Rust?

Rust is a multi-paradigm, open-source programming language that prioritizes performance and safety,
particularly in concurrency, and can be used for a range of applications, including operating systems,
browsers, and embedded devices. It offers both high-level and low-level abstractions to make programming
more efficient and secure.

### How Rust OOP works?

Rust is not a pure OOP language. It's a mix of OOP and functional programming. Rust uses a concept called **traits**,
similar to interfaces in other languages, but are also able to provide default method implementations,
generic methods, and associated types. <br>
<br>
There is no inheritance in Rust, but you can use the **trait object** to simulate it. <br>

Example:

```Rust
trait Shape {
    fn draw(&self);

    fn hello_world(&self) {
        println!("Hello, world!");
    }
}

struct Circle {
    color: String
}

impl Shape for Circle {
    fn draw(&self) {
        println!("Je suis un cercle de couleur {}.", self.color);
    }
}

struct Square {
    color: String
}

impl Shape for Square {
    fn draw(&self) {
        println!("Je suis un carré de couleur {}.", self.color);
    }
}

fn main() {
    let mut shapes: Vec<Box<dyn Shape>> = Vec::new();
    shapes.push(Box::new(Circle { color: String::from("rouge") }));
    shapes.push(Box::new(Square { color: String::from("bleu") }));

    for shape in shapes.iter() {
        shape.draw();
        shape.hello_world();
    }
}
```
```bash
Je suis un cercle de couleur rouge.
Hello, world!
Je suis un carré de couleur bleu.
Hello, world!
```

## Isometric game

An isometric game is a game that uses an isometric projection. It's a way to represent a 3D world on a 2D
screen. It's used in many games (particularly Old School RPGs) to give a 3D feeling to the game.

There is no tricky math to do to make an isometric game. You just have to draw the textures at the right
position. <br>
<br>

> **Note**: Dofus is a good example of an isometric game.

---

## Diagram

---
## Installation

### Libraries

- [ggez](https://ggez.rs/)
- [rand](https://crates.io/crates/rand)
- [lazy_static](https://crates.io/crates/lazy_static)

### Build

```bash
git clone https://github.com/GuillaumeMCK/Rust-RPG.git
cd Rust-RPG
cargo build # or cargo run to build and run the game
```

---

## Credits

- Music:
- Atlas:
- Fonts:
- Sounds:

## License

[MIT](https://choosealicense.com/licenses/mit/)
