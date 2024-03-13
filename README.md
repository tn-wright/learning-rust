# Learning Rust

A collection of small projects as I learn rust

## Hello Rust

The hello world program covered in the Rust [book](https://doc.rust-lang.org/book/ch01-02-hello-world.html)

## Guesssing Game

The simple guessing game that is outlined in the Rust [book](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

## Todo List

A custom interactive CLI tool to manage a to-do list. It allows you to add and edit tasks, mark them as completed, and has a few different ways to remove tasks from the list. Created using the [console](https://crates.io/crates/console) and [dialoguer](https://crates.io/crates/dialoguer) crates to make the tool interactive and pleasant to use.

The next step is to integrate the tool with A SQLite database so that tasks can be saved between executions of the program.

## Snake Game

Snake implemented with the [piston_window](https://crates.io/crates/piston_window) crate. Created following along to [this](https://www.youtube.com/watch?v=DnT_7M7L7vo&list=PLJbE2Yu2zumDD5vy2BuSHvFZU0a6RDmgb&ab_channel=TensorProgramming) video.

## Weather Generator

This is an implementation of [this](https://korbohned.de/product/4-seasons-weather-table/) RPG weather generator by KorbohneD. It uses a hex flower to ensure that the results are consistent and weather does not undergo large, sudden transitions that would be unrealistic. When the tool is started, you select one of four seasons. You may then generate another day of weather or change the current season. Changing season resets you to the center of the new season's hex flower.

This was intentionally designed to not make use of references or borrowing to help learn how ownership works.

Future tasks include specifying a starting weather condition when the application starts and randomizing which hex is chosen when the season is changed.
