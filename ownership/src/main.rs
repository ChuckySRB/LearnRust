// Ownership is a concept in Rust that manages memory allocation and deallocation.

// Ownership rules:
// 1. Each value in Rust has a variable that is called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.



fn main() {
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len); // Owner is s1
    let s2 = s1; // Trainsfered ownership to s2

    // println!("{}", s2); // Try s1 to see error
    
    // print_out_scope(&s2); // S1 is out of scope
}

fn print_out_scope(s: &String) {
    println!("{}", s1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}