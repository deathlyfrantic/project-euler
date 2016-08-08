/* find the sum of all even fibonacci numbers that are
 * less than 4,000,000
 */

#include <stdio.h>

#define LIMIT 4000000

int main() {

    int fib = 1,
        next_fib = 0,
        last_fib = 0,
        sum = 0;

    while(fib < LIMIT) {

        if(fib % 2 == 0) {
            sum += fib;
        }

        next_fib = fib + last_fib;
        last_fib = fib;
        fib = next_fib;
    }

    printf("sum -> %d\n", sum);
}
