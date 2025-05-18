fn is_palindrome(num: u64) -> bool {

    let num_str = num.to_string();
    
    let chars: Vec<char> = num_str.chars().collect();
    
    let len = chars.len();
    for i in 0..len / 2 {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    
    true
}

fn main() {
    use std::io::{self, Write};
    
    print!("Введіть число для перевірки на паліндром: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    match input.trim().parse::<u64>() {
        Ok(num) => {
            if is_palindrome(num) {
                println!("Число {} є паліндромом", num);
            } else {
                println!("Число {} не є паліндромом", num);
            }
        },
        Err(_) => println!("Помилка! Введіть коректне додатне ціле число."),
    }
}
