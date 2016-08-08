/* determine the largest prime factor of the number 600,851,475,143.
 * (a "prime factor" being a factor that is a prime number.)
 */

#include <stdio.h>
#include <math.h>

#define     TRUE    1
#define     FALSE   0
#define     NUMBER  600851475143
/* #define     NUMBER  13195 */

static int is_prime(long n);
static long get_start(long n);

int main()
{
    long number  = NUMBER,
         prime   = 0,
         factor1 = get_start(number),
         factor2 = factor1;

    while(factor1 > 1) {
        if(number % factor1 == 0 && number % factor2 == 0) {

            if(is_prime(factor1) && factor1 > prime) {
                prime = factor1;
            }

            if(is_prime(factor2) && factor2 > prime) {
                prime = factor2;
            }
        }

        do {
            factor1 -= 2;
            factor2 = number / factor1;
        } while(number % factor1 != 0 && number % factor2 != 0);
    }

    printf("largest prime is -> %ld\n", prime);
}

int is_prime(long n)
{
    long factor = get_start(n);

    if(n % 2 == 0) {
        return FALSE;
    }

    for(long i = factor; i > 1; i -= 2) {
        if(n % i == 0) {
            return FALSE;
        }
    }

    return TRUE;
}

long get_start(long n)
{
    long start = sqrt(n);

    if(start % 2 == 0) {
        start--;
    }

    return start;
}
