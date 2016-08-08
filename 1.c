/* find the sum of all ints less than 1,000 that are
 * evenly divisible by 3 or 5
 */

#include <stdio.h>

#define LIMIT 1000

int main() {

    int sum = 0;

    for(int i = 0; i < LIMIT; i++) {

        if(i % 3 == 0 || i % 5 == 0) {
            sum += i;
        }
    }

    printf("%d\n", sum);
}
