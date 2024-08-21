use std::io; // Import the standard I/O library for handling user input

fn main() {
    // Welcome message
    println!("Welcome to your Rust app for classifying grades");
    
    // Initialize a mutable vector to store grades (floating-point numbers)
    let mut notas = Vec::<f64>::new();

    // Infinite loop to gather user input
    loop {
        println!("Enter a grade (or type 'fin' to exit):");
        
        // Initialize a mutable string to store user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        // Check if the user wants to exit the loop
        if input.trim() == "fin" {
            break; // Exit the loop if 'fin' is entered
        }

        // Parse the input string into a floating-point number
        let nota: f64 = match input.trim().parse() {
            Ok(num) if num >= 0.0 && num <= 10.0 => num, // Valid grade between 0.0 and 10.0
            _ => {
                // Error message for invalid input
                println!("Please enter a valid grade between 0 and 10");
                continue; // Restart the loop for a new input
            }
        };

        // Add the valid grade to the vector
        notas.push(nota);
    }

    // Display all entered grades
    println!("Grades: {:?}", notas);

    // Classify each grade and print the classification
    println!("Grade Classification:");
    for nota in &notas {
        let clasificacion = match nota {
            0.0..=4.9 => "Fail",           // Grade between 0.0 and 4.9
            5.0..=6.9 => "Pass",           // Grade between 5.0 and 6.9
            7.0..=8.9 => "Notable",        // Grade between 7.0 and 8.9
            9.0..=10.0 => "Outstanding",   // Grade between 9.0 and 10.0
            _ => unreachable!(),           // This should never be reached
        };

        // Print the grade and its classification
        println!("Grade: {:.1} - {}", nota, clasificacion);
    }

    // Calculate and display the average grade of the group
    let suma: f64 = notas.iter().sum(); // Sum all grades in the vector
    let media: f64 = suma / notas.len() as f64; // Calculate the average

    println!("The group's average grade is: {:.2}", media); // Display the average
}




