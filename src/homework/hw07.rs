use std::io;

fn invert_case(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().collect::<String>()
            } else if c.is_uppercase() {
                c.to_lowercase().collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect()
}

fn main() {
    println!("Введіть текст для зміни регістру:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Помилка зчитування");

    let input = input.trim(); // Забираємо \n на кінці

    let result = invert_case(input);

    println!("Змінений текст: {}", result);
}
