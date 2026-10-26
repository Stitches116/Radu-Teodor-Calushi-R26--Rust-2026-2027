// Problem 1

fn is_prime(number: u32) -> bool {
    if number < 2 {
        return false;
    }

    let mut divisor = 2;

    while divisor * divisor <= number {
        if number.is_multiple_of(divisor) {
            return false;
        }

        divisor += 1;
    }

    true
}

fn next_prime(x: u16) -> Option<u16> {
    let mut number = u32::from(x) + 1;

    while number <= u32::from(u16::MAX) {
        if is_prime(number) {
            return Some(number as u16);
        }

        number += 1;
    }

    None
}

// Problem 2

fn add_with_panic(a: u32, b: u32) -> u32 {
    match a.checked_add(b) {
        Some(value) => value,
        None => panic!("Addition overflow"),
    }
}

fn multiply_with_panic(a: u32, b: u32) -> u32 {
    match a.checked_mul(b) {
        Some(value) => value,
        None => panic!("Multiplication overflow"),
    }
}

// Problem 3

#[derive(Debug)]
enum MathError {
    Overflow,
}

fn add_checked(a: u32, b: u32) -> Result<u32, MathError> {
    match a.checked_add(b) {
        Some(value) => Ok(value),
        None => Err(MathError::Overflow),
    }
}

fn multiply_checked(a: u32, b: u32) -> Result<u32, MathError> {
    match a.checked_mul(b) {
        Some(value) => Ok(value),
        None => Err(MathError::Overflow),
    }
}

fn calculate(a: u32, b: u32, c: u32) -> Result<u32, MathError> {
    let sum = add_checked(a, b)?;
    let product = multiply_checked(sum, c)?;

    Ok(product)
}

// Problem 4

#[derive(Debug)]
enum CharacterError {
    NotAscii,
    NotDigit,
    NotBase16Digit,
    NotLetter,
    NotPrintable,
}

fn to_uppercase(c: char) -> Result<String, CharacterError> {
    if !c.is_alphabetic() {
        return Err(CharacterError::NotLetter);
    }

    Ok(c.to_uppercase().collect())
}

fn to_lowercase(c: char) -> Result<String, CharacterError> {
    if !c.is_alphabetic() {
        return Err(CharacterError::NotLetter);
    }

    Ok(c.to_lowercase().collect())
}

fn print_char(c: char) -> Result<(), CharacterError> {
    if c.is_control() {
        return Err(CharacterError::NotPrintable);
    }

    println!("Character: {c}");
    Ok(())
}

fn char_to_number(c: char) -> Result<u32, CharacterError> {
    if !c.is_ascii() {
        return Err(CharacterError::NotAscii);
    }

    if !c.is_ascii_digit() {
        return Err(CharacterError::NotDigit);
    }

    match c.to_digit(10) {
        Some(number) => Ok(number),
        None => Err(CharacterError::NotDigit),
    }
}

fn char_to_number_hex(c: char) -> Result<u32, CharacterError> {
    if !c.is_ascii() {
        return Err(CharacterError::NotAscii);
    }

    if !c.is_ascii_hexdigit() {
        return Err(CharacterError::NotBase16Digit);
    }

    match c.to_digit(16) {
        Some(number) => Ok(number),
        None => Err(CharacterError::NotBase16Digit),
    }
}

fn print_error(error: CharacterError) {
    match error {
        CharacterError::NotAscii => {
            println!("Error: character is not ASCII")
        }
        CharacterError::NotDigit => {
            println!("Error: character is not a digit")
        }
        CharacterError::NotBase16Digit => {
            println!("Error: character is not a hexadecimal digit")
        }
        CharacterError::NotLetter => {
            println!("Error: character is not a letter")
        }
        CharacterError::NotPrintable => {
            println!("Error: character is not printable")
        }
    }
}

// Problem 5

#[derive(Debug)]
enum ShopError {
    ProductNotFound,
    TotalOverflow,
}

fn find_price(product: &str) -> Option<u32> {
    match product {
        "apple" => Some(5),
        "bread" => Some(8),
        "water" => Some(3),
        _ => None,
    }
}

fn buy_product(product: &str, quantity: u32) -> Result<u32, ShopError> {
    let price = match find_price(product) {
        Some(value) => value,
        None => return Err(ShopError::ProductNotFound),
    };

    match price.checked_mul(quantity) {
        Some(total) => Ok(total),
        None => Err(ShopError::TotalOverflow),
    }
}

fn main() {
    // Test panic cases
    if let Some(mode) = std::env::args().nth(1) {
        if mode == "panic-add" {
            add_with_panic(u32::MAX, 1);
            return;
        }

        if mode == "panic-mul" {
            multiply_with_panic(u32::MAX, 2);
            return;
        }
    }

    // Problem 1
    println!(" Problem 1 ");
    println!("Next prime after 10: {:?}", next_prime(10));
    println!("Next prime after 65521: {:?}", next_prime(65521));

    let mut current = 65480u16;

    loop {
        match next_prime(current) {
            Some(prime) => {
                println!("Prime: {prime}");
                current = prime;
            }
            None => {
                println!("No more prime numbers");
                break;
            }
        }
    }

    // Problem 2
    println!("\n Problem 2 ");
    println!("10 + 20 = {}", add_with_panic(10, 20));
    println!("6 * 7 = {}", multiply_with_panic(6, 7));

    // Problem 3
    println!("\n Problem 3 ");
    println!("Addition: {:?}", add_checked(10, 20));
    println!("Addition error: {:?}", add_checked(u32::MAX, 1));

    println!("Multiplication: {:?}", multiply_checked(6, 7));
    println!("Multiplication error: {:?}", multiply_checked(u32::MAX, 2));

    println!("Calculate: {:?}", calculate(10, 20, 2));
    println!("Calculate error: {:?}", calculate(u32::MAX, 1, 2));
    println!(
        "Calculate multiplication error: {:?}",
        calculate(1, 1, u32::MAX)
    );

    // Problem 4
    println!("\n Problem 4 ");

    println!("Uppercase a: {:?}", to_uppercase('a'));
    println!("Uppercase 5: {:?}", to_uppercase('5'));

    println!("Lowercase B: {:?}", to_lowercase('B'));
    println!("Lowercase 9: {:?}", to_lowercase('9'));

    println!("Print A: {:?}", print_char('A'));
    println!("Print newline: {:?}", print_char('\n'));

    println!("Digit 7: {:?}", char_to_number('7'));
    println!("Digit x: {:?}", char_to_number('x'));
    println!("Digit non-ASCII: {:?}", char_to_number('é'));

    println!("Hex F: {:?}", char_to_number_hex('F'));
    println!("Hex G: {:?}", char_to_number_hex('G'));
    println!("Hex non-ASCII: {:?}", char_to_number_hex('é'));

    print_error(CharacterError::NotAscii);
    print_error(CharacterError::NotDigit);
    print_error(CharacterError::NotBase16Digit);
    print_error(CharacterError::NotLetter);
    print_error(CharacterError::NotPrintable);

    // Problem 5
    println!("\n Problem 5 ");

    println!("Existing product: {:?}", find_price("apple"));
    println!("Missing product: {:?}", find_price("milk"));

    println!("Successful purchase: {:?}", buy_product("apple", 3));
    println!("Unknown product: {:?}", buy_product("milk", 2));
    println!("Too large total: {:?}", buy_product("apple", u32::MAX));
}
