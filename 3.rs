/* determine the largest prime factor of the number 600,851,475,143.
 * (a "prime factor" being a factor that is a prime number.)
 */

const NUMBER: f64 = 600851475143.0;

fn main() {
    let mut prime: i64 = 0;
    let mut factor1: i64 = get_start(NUMBER);
    let mut factor2: i64 = factor1;

    while factor1 > 1 {
        if NUMBER % factor1 as f64 == 0.0 && NUMBER % factor2 as f64 == 0.0 {
            if is_prime(factor1) && factor1 > prime {
                prime = factor1;
            }

            if is_prime(factor2) && factor2 > prime {
                prime = factor2;
            }
        }

        factor1 -= 2;
        factor2 = (NUMBER / factor1 as f64) as i64;

        while NUMBER % factor1 as f64 != 0.0 && NUMBER % factor2 as f64 != 0.0 {
            factor1 -= 2;
            factor2 = (NUMBER / factor1 as f64) as i64;
        }
    }

    println!("largest prime is -> {}", prime);
}

fn is_prime(n: i64) -> bool {
    let mut factor: i64 = get_start(n as f64);

    if n % 2 == 0 {
        return false
    }

    while factor > 1 {

        if n % factor == 0 {
            return false
        }

        factor -= 2;
    }

    true
}

fn get_start(n: f64) -> i64 {
    let x: f64 = n.sqrt();
    let mut start = x as i64;

    if start % 2 == 0 {
        start -= 1;
    }

    start
}
