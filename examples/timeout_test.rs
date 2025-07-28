use qsingleton::singleton;
use std::panic;
use std::time::Instant;

#[singleton(sleep_ms = 100)]
#[derive(Debug)]
struct TimeoutTest {
    data: String,
}

fn main() {
    println!("Testing timeout behavior...");
    
    let result = panic::catch_unwind(|| {
        let start = Instant::now();
        let _ = TimeoutTest::global(); // This should timeout and panic
        start.elapsed()
    });
    
    match result {
        Ok(_) => {
            println!("ERROR: Expected timeout but got success!");
            std::process::exit(1);
        }
        Err(panic_info) => {
            if let Some(message) = panic_info.downcast_ref::<String>() {
                println!("SUCCESS: Got expected panic message:");
                println!("{}", message);
                
                // Verify the message contains expected information
                if message.contains("TimeoutTest") && 
                   message.contains("not initialized") && 
                   message.contains("20 retries") &&
                   message.contains("100ms") {
                    println!("✓ Panic message contains all expected information");
                } else {
                    println!("✗ Panic message missing expected information");
                    std::process::exit(1);
                }
            } else {
                println!("ERROR: Panic payload is not a string");
                std::process::exit(1);
            }
        }
    }
    
    println!("Timeout test completed successfully!");
}