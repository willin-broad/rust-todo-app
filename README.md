# Rust Todo App

A command-line todo application built with Rust.

## Features

- Add new tasks
- Mark tasks as complete
- Mark tasks as pending
- List all tasks
- List only pending or completed tasks
- Delete tasks
- Persistent storage in `data/todos.json`

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2021 or later)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### Build the Project

```bash
cargo build
```

### Run the Application

You can run the app using Cargo:

```bash
cargo run -- <COMMAND> [OPTIONS]
```

Or, after building, run the binary directly:

```bash
./target/debug/todo <COMMAND> [OPTIONS]
```

## Usage

### Add a new todo

```bash
cargo run -- add "Buy groceries"
```

### List all todos

```bash
cargo run -- list
```

### List only pending todos

```bash
cargo run -- list --pending
```

### List only completed todos

```bash
cargo run -- list --completed
```

### Mark a todo as complete

```bash
cargo run -- complete 1
```

### Mark a todo as pending (uncomplete)

```bash
cargo run -- uncomplete 1
```

### Delete a todo

```bash
cargo run -- delete 1
```

## Data Storage

- Todos are stored in `data/todos.json` in the project directory.
- The data directory and file are created automatically if they do not exist.

## Running Tests

To run the unit tests:

```bash
cargo test
```

## Project Structure

```
rust-todo-app/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs
    ├── main.rs
    ├── storage.rs
    └── todo.rs
```

## Troubleshooting

- If you encounter errors about borrowing, ensure you are using Rust 2021 edition or later.
- If you get errors about missing dependencies, run `cargo build` to fetch them.
- If you want to reset your todo list, delete the `data/todos.json` file.
