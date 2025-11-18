use or_panic::ResultOrPanic;

fn main() {
    let duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .or_panic("System time is earlier than UNIX epoch");

    let secs = duration.as_secs();
    println!("Seconds since epoch: {secs}");
}
