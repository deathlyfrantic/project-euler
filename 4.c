/* find the largest palindromic number from the product of two
 * 3-digit numbers (100-999)
 */

#include <stdio.h>
#include <string.h>

#define     TRUE    1
#define     FALSE   0

static int is_palindrome(int n);

int main()
{
    int number1 = 999,
        number2,
        product,
        largest[] = {0, 0, 0};

    while(number1 > 99) {
        number2 = 999;

        while(number2 > 99) {
            product = number1 * number2;

            if(is_palindrome(product) && product > largest[0]) {
                largest[0] = product;
                largest[1] = number1;
                largest[2] = number2;
            }

            number2--;
        }

        number1--;
    }

    printf("%d is palindrome from factors %d and %d\n", largest[0], largest[1], largest[2]);
}

int is_palindrome(int n)
{
    // 999 x 999 is 6 digits, hence a length of 6 (+ 1 for \0)
    char num_str[7];
    int length;

    sprintf(num_str, "%d", n);
    length = strlen(num_str);

    for(int i = 0; i < length; i++) {
        if(num_str[i] != num_str[length - 1 - i]) {
            return FALSE;
        }
    }

    return TRUE;
}
