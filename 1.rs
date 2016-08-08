/* find the sum of all ints less than 1,000 that are
 * evenly divisible by 3 or 5
 */

fn main() {
    let mut sum: i32 = 0;

    for x in 0..1000 {
        if x % 3 == 0 || x % 5 == 0 {
            sum += x;
        }
    }

    println!("sum -> {}", sum);
}
