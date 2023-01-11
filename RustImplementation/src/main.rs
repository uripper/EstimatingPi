use rand::Rng; 
use std::time::Instant;

pub fn main() {
    let mut rng = rand::thread_rng();
    let mut co_prime_count: f64 = 1.0;
    let mut total_count: f64 = 1.0;
    let start = Instant::now();
    (1..1000000000000).for_each(|_i: i64| {
        let mut a = rng.gen::<i64>();
        let mut b = rng.gen::<i64>();
        if a < b {
            let temp = a;
            a = b;
            b = temp;
        }
        if gcd(a, b) == 1 {
            co_prime_count += 1.0;
        };
        total_count += 1.0;
    });
    let elapsed = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", elapsed);
    let ratio: f64 =  total_count/ co_prime_count;
    let pi: f64 = ratio * 6.0;
    let pi = pi.sqrt();
    println!("Total count is {}", total_count);
    println!("Co prime count is {}", co_prime_count);
    println!("Ratio is {}", ratio);
    println!("Pi is {}", pi);

}

fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 {
        return b
    } else {
        return gcd(b % a, a)
    }
}


