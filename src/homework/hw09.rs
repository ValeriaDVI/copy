fn rotate(s: String, n: isize) -> String {
    if s.is_empty() {
        return s;
    }

    let len = s.len();
    let normalized_shift = ((n % len as isize) + len as isize) % len as isize;
    
    if normalized_shift == 0 {
        return s;
    }

    let chars: Vec<char> = s.chars().collect();
    let shift_index = if normalized_shift > 0 { 
        len - normalized_shift as usize 
    } else { 
        (-normalized_shift) as usize 
    };
    
    let mut result = String::with_capacity(len);
    result.extend(chars[shift_index..].iter());
    result.extend(chars[..shift_index].iter());
    
    result
}

fn main() {
    use std::io::{self, Write};
    
    print!("Введіть рядок: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_string();
    
    print!("Введіть кількість позицій для зсуву (додатнє число - вправо, від'ємне - вліво): ");
    io::stdout().flush().unwrap();
    let mut shift_str = String::new();
    io::stdin().read_line(&mut shift_str).unwrap();
    let shift: isize = shift_str.trim().parse().unwrap_or(0);
    
    let result = rotate(input.clone(), shift);
    println!("Результат зсуву: {}", result);
}
