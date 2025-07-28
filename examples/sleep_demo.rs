use qsingleton::singleton;
use std::thread;
use std::time::{Duration, Instant};

// Basic singleton with default 500ms sleep
#[singleton]
#[derive(Debug)]
struct DefaultSleep {
    value: String,
}

// Singleton with custom 1000ms sleep
#[singleton(sleep_ms = 1000)]
#[derive(Debug)]
struct CustomSleep {
    value: String,
}

// Arc-based singleton with custom 200ms sleep
#[singleton(arc, sleep_ms = 200)]
#[derive(Debug)]
struct ArcCustomSleep {
    value: String,
}

fn main() {
    println!("Demonstrating sleep functionality in qsingleton");
    println!("==============================================");

    // Test 1: Default sleep behavior (500ms)
    println!("\n1. Testing default sleep (500ms):");
    
    let handle1 = thread::spawn(|| {
        println!("Thread 1: Attempting to access DefaultSleep (should sleep and wait)");
        let start = Instant::now();
        let instance = DefaultSleep::global();
        let elapsed = start.elapsed();
        println!("Thread 1: Got instance after {:?}: {}", elapsed, instance.value);
    });
    
    // Initialize after a delay to test sleep behavior
    thread::sleep(Duration::from_millis(750));
    println!("Main: Initializing DefaultSleep after 750ms delay");
    DefaultSleep::init(DefaultSleep {
        value: "Default sleep singleton initialized!".to_string(),
    });
    
    handle1.join().unwrap();

    // Test 2: Custom sleep behavior (1000ms)
    println!("\n2. Testing custom sleep (1000ms):");
    
    let handle2 = thread::spawn(|| {
        println!("Thread 2: Attempting to access CustomSleep (should sleep longer)");
        let start = Instant::now();
        let instance = CustomSleep::global();
        let elapsed = start.elapsed();
        println!("Thread 2: Got instance after {:?}: {}", elapsed, instance.value);
    });
    
    // Initialize after a delay to test custom sleep behavior
    thread::sleep(Duration::from_millis(1500));
    println!("Main: Initializing CustomSleep after 1500ms delay");
    CustomSleep::init(CustomSleep {
        value: "Custom sleep singleton initialized!".to_string(),
    });
    
    handle2.join().unwrap();

    // Test 3: Arc with custom sleep (200ms)
    println!("\n3. Testing Arc with custom sleep (200ms):");
    
    let handle3 = thread::spawn(|| {
        println!("Thread 3: Attempting to access ArcCustomSleep (fast sleep)");
        let start = Instant::now();
        let instance = ArcCustomSleep::global();
        let elapsed = start.elapsed();
        println!("Thread 3: Got Arc instance after {:?}: {}", elapsed, instance.value);
    });
    
    // Initialize after a short delay
    thread::sleep(Duration::from_millis(300));
    println!("Main: Initializing ArcCustomSleep after 300ms delay");
    ArcCustomSleep::init(ArcCustomSleep {
        value: "Arc custom sleep singleton initialized!".to_string(),
    });
    
    handle3.join().unwrap();

    // Test 4: Multiple threads accessing the same initialized singleton
    println!("\n4. Testing multiple threads accessing initialized singleton:");
    
    let handles: Vec<_> = (0..3)
        .map(|i| {
            thread::spawn(move || {
                let start = Instant::now();
                let instance = DefaultSleep::global();
                let elapsed = start.elapsed();
                println!("Thread {}: Got instance immediately after {:?}: {}", i, elapsed, instance.value);
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }

    println!("\nAll tests completed successfully!");
}