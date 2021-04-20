pub fn run() {
    // Print to console
    println!("Hello from the print.rs");

    // Basic Formatting
    println!("{} is from {}", "Kashyap", "India");

    //Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Kashyap", "India", "Code"
    );

    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "John",
        activity = "Cricket "
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octcal: {:o}", 10, 10, 10);

    // Placeholder for debug traits
    println!("{:?}", (12, true, "herllo"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
