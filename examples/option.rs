use or_panic::OptionOrPanic;

fn main() {
    println!("Example: Getting the highest priority task");

    let tasks = [
        ("clean", 1),
        ("deploy", 10),
        ("backup", 3),
    ];

    let next_task = tasks
        .iter()
        .max_by_key(|(_, priority)| *priority)
        .or_panic("No tasks available");

    println!("Next task: {} (priority {})", next_task.0, next_task.1);
}
