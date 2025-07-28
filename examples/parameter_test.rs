use qsingleton::singleton;

// Test various parameter combinations

// Just sleep_ms parameter
#[singleton(sleep_ms = 250)]
#[derive(Debug)]
struct OnlySleep {
    data: i32,
}

// Arc with sleep_ms
#[singleton(arc, sleep_ms = 750)]
#[derive(Debug)]
struct ArcWithSleep {
    data: String,
}

// Default parameters (should use 500ms)
#[singleton]
#[derive(Debug)]
struct DefaultParams {
    data: bool,
}

// Just arc (should use 500ms default)
#[singleton(arc)]
#[derive(Debug)]
struct JustArc {
    data: f64,
}

fn main() {
    println!("Testing parameter parsing combinations");
    
    // Initialize all singletons
    OnlySleep::init(OnlySleep { data: 1 });
    ArcWithSleep::init(ArcWithSleep { data: "test".to_string() });
    DefaultParams::init(DefaultParams { data: true });
    JustArc::init(JustArc { data: 3.14 });
    
    // Access them
    let only_sleep = OnlySleep::global();
    println!("OnlySleep: {}", only_sleep.data);
    
    let arc_with_sleep = ArcWithSleep::global();
    println!("ArcWithSleep: {}", arc_with_sleep.data);
    
    let default_params = DefaultParams::global();
    println!("DefaultParams: {}", default_params.data);
    
    let just_arc = JustArc::global();
    println!("JustArc: {}", just_arc.data);
    
    println!("All parameter combinations work correctly!");
}