use rand::Rng; 
use std::time::Instant;

pub fn main() {
    let mut rng = rand::thread_rng();
    let mut co_prime_count: f64 = 1.0;
    let mut total_count: f64 = 1.0;
    let start = Instant::now();
    (1..100000000).for_each(|_i: i64| {
        let mut a = rng.gen::<i64>();
        let mut b = rng.gen::<i64>();
        // Make sure the numbers are positive
        if a < 0 {
            a = a * -1;
        }
        if b < 0 {
            b = b * -1;
        }
        if a < b {
            let temp = a;
            a = b;
            b = temp;
        }
        if a == 0 || b == 0 {
            a = a + 1;
            b = b + 1;
        }

        if gcd(a, b) == 1 {
            co_prime_count += 1.0;
        };
        total_count += 1.0;
    });
    let elapsed = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", elapsed);
    println!("Total count is {}", total_count);
    println!("Co prime count is {}", co_prime_count);
    let ratio: f64 =  total_count/ co_prime_count;
    println!("Ratio is {}", ratio);
    let pi: f64 = ratio * 6.0;
    println!("ratio * 6 is {}", pi);
    let pi = pi.sqrt();

    
    println!("Pi is {}", pi);

}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a
    } else {
        return gcd(b, a % b)
    }
}


