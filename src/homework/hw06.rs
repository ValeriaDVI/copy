use std::io;

fn main() {
    let mut input = String::new();
    println!("Введіть кількість трикутників:");

    io::stdin()
        .read_line(&mut input)
        .expect("Помилка зчитування");

    let triangles: usize = input.trim().parse().expect("Введіть число!");

    let max_width = 2 * triangles + 1; // Ширина найнижчого рядка

    for level in 1..=triangles {
        for row in 1..=level {
            let stars = 2 * row - 1;
            let line = "*".repeat(stars);
            println!("{:^width$}", line, width = max_width);
        }
    }
}
