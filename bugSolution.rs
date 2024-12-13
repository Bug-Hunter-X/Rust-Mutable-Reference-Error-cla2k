fn main() {
    let mut x = 5;
    { //Adding a scope to create a single mutable reference at a time
        let y = &mut x;
        *y = 10;
    }

    { //Adding a scope to create a single mutable reference at a time
        let z = &mut x; 
        *z = 15;
    }
    println!("x = {}", x);
}
