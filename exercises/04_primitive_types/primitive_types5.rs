fn main() {
    let cat = ("Furry McFurson", 3.5);
    //  ^^^ Creates a tuple with 2 elements:
    //      - First element: "Furry McFurson" (string slice &str)
    //      - Second element: 3.5 (floating point number f64)

    let (name, age) = cat;
    //  ^^^^^^^^^^^^^ This is tuple destructuring
    //  - Takes the tuple `cat` and breaks it apart
    //  - `name` gets the first element: "Furry McFurson"
    //  - `age` gets the second element: 3.5
    //  - Both variables are now available for use

    println!("{name} is {age} years old");
    //        ^^^^^^     ^^^^^ These use the destructured variables
    //        Uses `name` variable (contains "Furry McFurson")
    //        Uses `age` variable (contains 3.5)
}
