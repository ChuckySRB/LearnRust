// Funtions

// fn snake_case


fn main() {
    zdravo_svete();    

    let sisa_sam = kolika_si_sisa(9555);
    println!("{}", sisa_sam);
}


fn zdravo_svete() {
    println!("Zdravo Svete!");
}

// cast operation:

fn kolika_si_sisa(sisa: i32) -> String{
    if sisa < 10 {
        return "Mala sisa".to_string();
    } else if sisa < 20 {
        return "Srednja sisa".to_string();
    } 
    else if sisa > MAX_SISA as i32 {
        return "IT'S OVER 9000!".to_string();
    }
    else {
        return "Velika sisa".to_string();
    }
}

// Expressions and Statements

// Statements are instructions that perform actions and do not return a value.
// Expressions are instructions that return a value.

// Statements:
// let x = 5;
// fn zdravo_svete() {
//     println!("Zdravo Svete!");
// }

// Expressions:
// 5
// zdravo_svete()
// x + 1

const MAX_SISA: i64 = 9000;




