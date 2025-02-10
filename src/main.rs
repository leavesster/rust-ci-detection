fn main() {
    println!("Hello, world!");

    if std::env::var("CI").is_ok() {
        println!("Running in CI");
    } else {
        println!("Not running in CI");
    }
}
