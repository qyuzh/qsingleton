use qsingleton::singleton;

// Basic singleton returning &'static Self
#[singleton]
#[derive(Debug)]
struct Config {
    name: String,
    version: String,
}

// Arc-based singleton returning Arc<Self>
#[singleton(arc)]
#[derive(Debug)]
struct Database {
    connection_string: String,
    pool_size: usize,
}

fn main() {
    // Initialize the Config singleton
    Config::init(Config {
        name: "MyApp".to_string(),
        version: "1.0.0".to_string(),
    });

    // Initialize the Database singleton
    Database::init(Database {
        connection_string: "postgresql://localhost/mydb".to_string(),
        pool_size: 10,
    });

    // Access the singletons
    let config = Config::global();
    println!("App: {} v{}", config.name, config.version);

    let db = Database::global();
    println!("DB: {} (pool size: {})", db.connection_string, db.pool_size);

    // Arc can be cloned and moved
    let db_clone = Database::global();
    std::thread::spawn(move || {
        println!("In thread: {}", db_clone.connection_string);
    })
    .join()
    .unwrap();
}
