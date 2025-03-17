
// Unsigned and signed
// Unsigned: u8, u16, u32, u64, u128
// Signed: i8, i16, i32, i64, i128

// Floating point: f32, f64

// Boolean: bool

// Character: char

// Tuple: (i32, bool)

// Array: [i32; 5]

// String: String
// string slice: &str

fn main() {
    let x: i32 = 10;
    let y: i32 = 20;
    println!("x: {}, y: {}", x, y);
    let _unsiged_x: u128 = 0;
    // let z = unsiged_x - 1;
    // println!("z: {}", z);
    let gey_ok: bool = false;
    if gey_ok {
        println!("gey is ok");
    } else {
        println!("gey is not ok");
    }

    let slovo: char = 'Ч';
    let slovo2: char = 'Ћ';
    
    println!("Slova {} {}", slovo, slovo2);

    let djole: (String, bool) = ("Djole".to_string(), true);
    println!("{} is {}", djole.0, djole.1);

    let braca: [i32; 5] = [1, 2, 3, 4, 5];
    println!("braca: {:?}", braca);

    // Slices
    let slice: &[i32] = &braca[2..4];
    println!("slice: {:?}", slice);

    let mut govor: String = String::from("Nesto");
    govor.push_str(" je dobar");
    println!("{}", govor);
    
}
