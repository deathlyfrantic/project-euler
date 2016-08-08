/* 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
 * What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
 */

fn main() {
    let mut smallest: i32 = 1;

    while !divisible(smallest) {
        smallest += 1;
    }

    println!("smallest number is -> {}", smallest);
}

fn divisible(n: i32) -> bool {
    // don't have to test 1 since every number is evenly divisible by 1
    for i in 2..21 {
        if n % i != 0 {
            return false
        }
    }

    true
}
