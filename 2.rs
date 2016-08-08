/* find the sum of all even fibonacci numbers that are
 * less than 4,000,000
 */

const LIMIT: i32 = 4000000;

fn main() {
    let mut fib: i32 = 1;
    let mut next_fib: i32;
    let mut last_fib: i32 = 0;
    let mut sum: i32 = 0;

    while fib < LIMIT {
        if fib % 2 == 0 {
            sum += fib;
        }

        next_fib = fib + last_fib;
        last_fib = fib;
        fib = next_fib;
    }

    println!("sum -> {}", sum);
}
