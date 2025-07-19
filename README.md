# QSingleton

A Rust procedural macro library for implementing thread-safe singleton patterns with minimal boilerplate.

## Features

- **Thread-safe**: Uses `std::sync::OnceLock` for safe concurrent access
- **Flexible return types**: Choose between `&'static Self` or `Arc<Self>`
- **Simple API**: Just two methods - `init()` and `global()`
- **Zero runtime overhead**: All safety guarantees are compile-time

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
qsingelton = "0.1.0"
```

## Usage

### Basic Singleton (returns `&'static Self`)

```rust
use qsingelton::singleton;

#[singleton]
struct Config {
    name: String,
    version: String,
}

fn main() {
    // Initialize the singleton
    Config::init(Config {
        name: "MyApp".to_string(),
        version: "1.0.0".to_string(),
    });

    // Access the global instance
    let config = Config::global();
    println!("App: {} v{}", config.name, config.version);
}
```

### Arc-based Singleton (returns `&'static Arc<Self>`)

```rust
use qsinglton::singleton;

#[singleton(arc)]
struct Database {
    connection_string: String,
    pool_size: usize,
}

fn main() {
    // Initialize the singleton
    Database::init(Database {
        connection_string: "postgresql://localhost/mydb".to_string(),
        pool_size: 10,
    });

    // Access the global instance (can be cloned and moved)
    let db = Database::global();
    let db_clone = db.clone();
    
    std::thread::spawn(move || {
        println!("In thread: {}", db_clone.connection_string);
    }).join().unwrap();
}
```

## API

### `init(instance: Self)`

Initializes the singleton with the provided instance. Panics if called more than once.

### `global() -> &'static Self` or `global() -> &'static Arc<Self>`

Returns a reference to the global singleton instance. Panics if called before `init()`.

## Thread Safety

All generated code is thread-safe and uses `std::sync::OnceLock` internally to ensure:

- Only one initialization can succeed
- Multiple threads can safely access the singleton
- No data races or undefined behavior

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
