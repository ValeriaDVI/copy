use std::io::{self, Write};

struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    a: Point,
    b: Point,
}

impl Rectangle {
    fn area(&self) -> i32 {
        (self.b.x - self.a.x).abs() * (self.b.y - self.a.y).abs()
    }
    
    fn intersects(&self, other: &Rectangle) -> bool {
        !(self.a.x > other.b.x || self.b.x < other.a.x || 
          self.a.y < other.b.y || self.b.y > other.a.y)
    }
    
    fn intersection(&self, other: &Rectangle) -> Option<Rectangle> {
        if !self.intersects(other) {
            return None;
        }
        
        let x_left = self.a.x.max(other.a.x);
        let y_top = self.a.y.min(other.a.y);
        let x_right = self.b.x.min(other.b.x);
        let y_bottom = self.b.y.max(other.b.y);
        
        Some(Rectangle {
            a: Point { x: x_left, y: y_top },
            b: Point { x: x_right, y: y_bottom },
        })
    }
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let n = xs.len();
    
    if n == 0 {
        return 0;
    }
    
    if n == 1 {
        return xs[0].area();
    }
    
    let mut total_area = 0;
    
    for i in 0..n {
        total_area += xs[i].area();
        
        for j in (i+1)..n {
            if let Some(intersection) = xs[i].intersection(&xs[j]) {
                total_area -= intersection.area();
            }
        }
    }
    
    for i in 0..n {
        for j in (i+1)..n {
            for k in (j+1)..n {
                if let Some(ij) = xs[i].intersection(&xs[j]) {
                    if let Some(ijk) = ij.intersection(&xs[k]) {
                        total_area += ijk.area();
                    }
                }
            }
        }
    }
    
    total_area
}

fn read_i32() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Помилка при читанні вводу");
    input.trim().parse().expect("Введено некоректне число")
}

fn read_rectangle() -> Rectangle {
    print!("Введіть координату x верхньої лівої точки: ");
    io::stdout().flush().unwrap();
    let x1 = read_i32();
    
    print!("Введіть координату y верхньої лівої точки: ");
    io::stdout().flush().unwrap();
    let y1 = read_i32();
    
    print!("Введіть координату x нижньої правої точки: ");
    io::stdout().flush().unwrap();
    let x2 = read_i32();
    
    print!("Введіть координату y нижньої правої точки: ");
    io::stdout().flush().unwrap();
    let y2 = read_i32();
    
    Rectangle {
        a: Point { x: x1, y: y1 },
        b: Point { x: x2, y: y2 },
    }
}

fn main() {
    print!("Скільки прямокутників ви хочете ввести? ");
    io::stdout().flush().unwrap();
    let n = read_i32() as usize;
    
    let mut rectangles = Vec::with_capacity(n);
    
    for i in 0..n {
        println!("\nПрямокутник {}:", i+1);
        rectangles.push(read_rectangle());
    }
    
    println!("\nПлощі окремих прямокутників:");
    for (i, rect) in rectangles.iter().enumerate() {
        println!("Прямокутник {}: {}", i+1, rect.area());
    }
    
    let occupied = area_occupied(&rectangles);
    println!("\nФактична зайнята площа: {}", occupied);
}
