fn main() {
    const W: usize = 30;
    const H: usize = 20;
    
    if W < 10 || W > 80 || H < 10 || H > 80 {
        println!("Розмір конверта має бути в діапазоні 10...80");
        return;
    }
    
    let mut result = String::new();
    
    for i in 0..H {
        for j in 0..W {
            if i == 0 || i == H - 1 {
                result.push('*');
            }
            else if j == 0 || j == W - 1 {
                result.push('*');
            }
            else if i == j {
                result.push('*');
            }
            else if j == W - 1 - i {
                result.push('*');
            }
            else {
                result.push(' ');
            }
        }
        
        if i < H - 1 {
            result.push('\n');
        }
    }

    println!("{}", result);
}
