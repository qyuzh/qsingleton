use qsingleton::singleton;
use std::thread;
use std::time::Duration;

// Basic singleton with default 500ms sleep
#[singleton]
#[derive(Debug)]
struct Config {
    name: String,
    version: String,
}

fn main() {
    println!("QSingleton Sleep Feature Example");
    println!("=================================");

    // Spawn a thread that tries to access the singleton before it's initialized
    let config_thread = thread::spawn(|| {
        println!("Thread: Trying to access Config singleton...");
        println!("Thread: This will sleep 500ms intervals until initialization completes");
        let config = Config::global();
        println!("Thread: Success! Got config: {} v{}", config.name, config.version);
    });

    // Simulate some initialization work
    println!("Main: Doing some initialization work for 1.2 seconds...");
    thread::sleep(Duration::from_millis(1200));
    
    // Initialize the singleton
    println!("Main: Initializing Config singleton");
    Config::init(Config {
        name: "MyApp".to_string(),
        version: "2.0.0".to_string(),
    });

    // Wait for the thread to complete
    config_thread.join().unwrap();

    // Show that subsequent accesses are immediate
    println!("Main: Accessing Config again (should be immediate)");
    let config = Config::global();
    println!("Main: Got config immediately: {} v{}", config.name, config.version);

    println!("\nExample completed successfully!");
}