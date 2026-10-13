
fn add_space(s: &mut String, n: usize) {
    for _ in 0..n {
        s.push(' ');
    }
}

fn add_str(s: &mut String, text: &str) {
    s.push_str(text);
}

fn add_integer(s: &mut String, mut n: u64) {
    if n == 0 {
        s.push('0');
        return;
    }

    let mut digits = String::new();
    let mut count = 0;

    while n > 0 {
        let digit = (n % 10) as u8;
        digits.push((b'0' + digit) as char);
        n /= 10;
        count += 1;

        if count % 3 == 0 && n > 0 {
            digits.push('_');
        }
    }

    for c in digits.chars().rev() {
        s.push(c);
    }
}

fn add_float(s: &mut String, n: f64) {
    let number = (n * 1000.0 + 0.5) as u64;

    let whole = number / 1000;
    let decimal = number % 1000;

    add_integer(s, whole);
    s.push('.');

    let a = (decimal / 100) as u8;
    let b = ((decimal / 10) % 10) as u8;
    let c = (decimal % 10) as u8;

    s.push((b'0' + a) as char);
    s.push((b'0' + b) as char);
    s.push((b'0' + c) as char);
}

fn main() {
    let mut s = String::new();

    add_space(&mut s, 30);
    add_str(&mut s, "I 💚");
    s.push('\n');

    add_space(&mut s, 30);
    add_str(&mut s, "RUST.");
    s.push('\n');
    s.push('\n');

    add_space(&mut s, 4);
    add_str(&mut s, "Most");
    add_space(&mut s, 12);
    add_str(&mut s, "crate");
    add_space(&mut s, 7);
    add_integer(&mut s, 306437968);
    add_space(&mut s, 9);
    add_str(&mut s, "and");
    add_space(&mut s, 6);
    add_str(&mut s, "lastest");
    add_space(&mut s, 7);
    add_str(&mut s, "is");
    s.push('\n');

    add_space(&mut s, 8);
    add_str(&mut s, "downloaded");
    add_space(&mut s, 7);
    add_str(&mut s, "has");
    add_space(&mut s, 8);
    add_str(&mut s, "downloads");
    add_space(&mut s, 5);
    add_str(&mut s, "the");
    add_space(&mut s, 9);
    add_str(&mut s, "version");
    add_space(&mut s, 4);
    add_float(&mut s, 2.038);
    add_str(&mut s, ".");

    println!("{}", s);
}
