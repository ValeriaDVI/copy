use rand::Rng;


fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let avg = rng.gen_range(10..30);
    let total = avg * n as u32;

    let mut shipments = Vec::with_capacity(n);
    let mut sum = 0;

    for _ in 0..n - 1 {
        let max_val = total.saturating_sub(sum).saturating_sub(5 * ((n - shipments.len()) as u32));
        let val = rng.gen_range(5..=max_val.max(5));
        shipments.push(val);
        sum += val;
    }


    shipments.push(total.saturating_sub(sum));
    shipments
}


fn count_permutation(shipments: &Vec<u32>) -> Result<usize, &'static str> {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total % n != 0 {
        return Err("❌ Розподіл неможливий.");
    }

    let avg = total / n;
    let moves = shipments
        .iter()
        .filter(|&&x| x > avg)
        .map(|&x| (x - avg) as usize)
        .sum();

    Ok(moves)
}

fn main() {
    let shipments = gen_shipments(6);
    println!("Вантажі: {:?}", shipments);

    match count_permutation(&shipments) {
        Ok(moves) => println!("Мін. переміщень: {}", moves),
        Err(msg) => println!("{}", msg),
    }
}
