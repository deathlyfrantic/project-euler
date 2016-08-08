/* 1^2 + 2^2 + ... + 10^2 = 385
 *
 * The square of the sum of the first ten natural numbers is, (1 + 2 + ... + 10)^2 = 552 = 3025
 *
 * Hence the difference between the sum of the squares of the first ten natural numbers and the
 * square of the sum is 3025 − 385 = 2640.
 *
 * Find the difference between the sum of the squares of the first one hundred natural numbers and
 * the square of the sum.
 */

fn main() {
    let mut sum_of_sq: i32 = 0;
    let mut sq_of_sum: i32 = 0;

    for i in 1..101 {
        let j = i as i32;
        sum_of_sq += j.pow(2);
        sq_of_sum += i;
    }

    sq_of_sum = sq_of_sum.pow(2);

    println!("sum of squares -> {}", sum_of_sq);
    println!("square of sums -> {}", sq_of_sum);
    println!("difference -> {}", sq_of_sum - sum_of_sq);
}
