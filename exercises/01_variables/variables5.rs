fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    let number = 3;
    println!("Number plus two is: {}", number + 2);
}

//  Concept Explanation: Variable Shadowing
// The solution uses variable shadowing, which is a powerful Rust feature that allows you to:

// --> Declare a new variable with the same name as a previous variable

// --> Change the type of the "variable" (technically creating a new one)

// --> Keep the same name while transforming the data
