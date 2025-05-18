use std::io;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Введіть перше число:");
    io::stdin()
        .read_line(&mut input1)
        .expect("Помилка зчитування");

    println!("Введіть друге число:");
    io::stdin()
        .read_line(&mut input2)
        .expect("Помилка зчитування");

    let num1: u64 = input1.trim().parse().expect("Введено не число");
    let num2: u64 = input2.trim().parse().expect("Введено не число");

    let result = gcd(num1, num2);

    println!("НСД (GCD) чисел {} і {} = {}", num1, num2, result);
}
