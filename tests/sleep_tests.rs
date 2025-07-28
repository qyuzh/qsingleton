use qsingleton::singleton;
use std::thread;
use std::time::{Duration, Instant};

#[singleton]
#[derive(Debug)]
struct TestSingleton {
    value: i32,
}

#[singleton(sleep_ms = 100)]
#[derive(Debug)]
struct CustomSleepSingleton {
    value: String,
}

#[singleton(arc, sleep_ms = 50)]
#[derive(Debug)]
struct ArcSleepSingleton {
    data: u64,
}

#[test]
fn test_sleep_functionality() {
    // Test default sleep behavior
    let handle = thread::spawn(|| {
        let start = Instant::now();
        let instance = TestSingleton::global();
        let elapsed = start.elapsed();
        assert_eq!(instance.value, 42);
        // Should have waited at least one sleep cycle (500ms)
        assert!(elapsed >= Duration::from_millis(400));
    });
    
    // Initialize after a delay
    thread::sleep(Duration::from_millis(600));
    TestSingleton::init(TestSingleton { value: 42 });
    
    handle.join().unwrap();
}

#[test]
fn test_custom_sleep_duration() {
    let handle = thread::spawn(|| {
        let start = Instant::now();
        let instance = CustomSleepSingleton::global();
        let elapsed = start.elapsed();
        assert_eq!(instance.value, "test");
        // Should have waited at least one sleep cycle (100ms)
        assert!(elapsed >= Duration::from_millis(80));
    });
    
    // Initialize after a delay
    thread::sleep(Duration::from_millis(150));
    CustomSleepSingleton::init(CustomSleepSingleton {
        value: "test".to_string(),
    });
    
    handle.join().unwrap();
}

#[test]
fn test_arc_sleep_functionality() {
    let handle = thread::spawn(|| {
        let start = Instant::now();
        let instance = ArcSleepSingleton::global();
        let elapsed = start.elapsed();
        assert_eq!(instance.data, 123);
        // Should have waited at least one sleep cycle (50ms)
        assert!(elapsed >= Duration::from_millis(40));
    });
    
    // Initialize after a delay
    thread::sleep(Duration::from_millis(75));
    ArcSleepSingleton::init(ArcSleepSingleton { data: 123 });
    
    handle.join().unwrap();
}

#[test]
fn test_immediate_access_after_init() {
    // This test uses different singleton instances to avoid conflicts
    
    #[singleton]
    #[derive(Debug)]
    struct ImmediateSingleton {
        name: String,
    }
    
    // Initialize first
    ImmediateSingleton::init(ImmediateSingleton {
        name: "immediate".to_string(),
    });
    
    // Access should be immediate
    let start = Instant::now();
    let instance = ImmediateSingleton::global();
    let elapsed = start.elapsed();
    
    assert_eq!(instance.name, "immediate");
    // Should be nearly instant (less than 10ms)
    assert!(elapsed < Duration::from_millis(10));
}