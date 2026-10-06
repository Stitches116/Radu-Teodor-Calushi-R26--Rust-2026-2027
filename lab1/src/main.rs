// problem 1
fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    let limit = (n as f64).sqrt() as u32;
    for i in 2..=limit {
        if n.is_multiple_of(i) {
            return false;
        }
    }
    true
}

// problem 2
fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a 
}

fn are_coprime(a: u32, b: u32) -> bool {
    gcd(a, b) == 1
}

// problem 3
fn sing_99_bottles() {
    for i in (1..=99).rev() {
        if i > 2 {
            println!("{0} bottles of beer on the wall, {0} bottles of beer.", i);
            println!("Take one down and pass it around, {} bottles of beer on the wall.\n", i - 1);
        } else if i == 2 {
            println!("2 bottles of beer on the wall, 2 bottles of beer.");
            println!("Take one down and pass it around, 1 bottle of beer on the wall.\n");
        } else {
            println!("1 bottle of beer on the wall, 1 bottle of beer.");
            println!("Take one down and pass it around, no more bottles of beer on the wall.\n");
        }
    }

    println!("No more bottles of beer on the wall, no more bottles of beer.");
    println!("Go to the store and buy some more, 99 bottles of beer on the wall.");
}

fn main() {
    println!("  problem 1 ");
    for number in 0..=100 {
        if is_prime(number) {
            print!("{} ", number);
        }
    }
    println!("\n"); 

    println!(" problem 2");
    
    let mut pairs_count = 0;
    for i in 0..=100 {
        for j in 0..=100 {
            if are_coprime(i, j) {
                pairs_count += 1;
            }
        }
    }
    println!("Found {} pairs of coprime numbers.\n", pairs_count);

    println!(" problem 3 ");
    sing_99_bottles();
}