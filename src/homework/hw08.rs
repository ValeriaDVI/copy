use std::io;

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    println!("Введіть число:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Помилка вводу");

    let num: u32 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Це не число.");
            return;
        }
    };

    if is_prime(num) {
        println!("Число {} є простим.", num);
    } else {
        println!("Число {} є складеним.", num);
    }
}
