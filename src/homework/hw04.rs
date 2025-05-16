fn main() {
    const W: usize = 11;
    const H: usize = 11;
    
    if W % 2 == 0 || H % 2 == 0 {
        println!("Розміри мають бути непарними для правильного ромба");
        return;
    }
    
    let mut result = String::new();
    
    let mid_w = W / 2;
    let mid_h = H / 2;
    
    for i in 0..H {
        let distance = if i <= mid_h {
            mid_h - i
        } else {
            i - mid_h
        };
        
        let stars = W - 2 * distance;
        
        for j in 0..W {
            if j >= distance && j < W - distance {
                result.push('*');
            } else {
                result.push(' ');
            }
        }
        
        if i < H - 1 {
            result.push('\n');
        }
    }

    println!("{}", result);
}
