/* find the largest palindromic number from the product of two
 * 3-digit numbers (100-999)
 */

fn main() {
    let mut num1: i32 = 999;
    let mut largest: (i32, i32, i32) = (0, 0, 0);

    while num1 > 99 {

        let mut num2: i32 = 999;

        while num2 > 99 {
            let product: i32 = num1 * num2;

            if is_palindrome(product) && product > largest.0 {
                largest = (product, num1, num2);
            }

            num2 -= 1;
        }

        num1 -= 1;
    }

    println!("largest palindromic num -> {} * {} = {}", largest.1, largest.2, largest.0);
}

fn is_palindrome(n: i32) -> bool {
    let s = n.to_string();
    let mut fwd_s = s.chars();
    let mut rev_s = s.chars().rev();
    let mut c = fwd_s.next();

    while c != None {
        if c != rev_s.next() {
            return false
        }

        c = fwd_s.next();
    }

    true
}
