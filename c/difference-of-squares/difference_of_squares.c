#include "difference_of_squares.h"

// (n(n+1)(2n+1))/6 Johann Faulhaber's formula
unsigned int sum_of_squares(unsigned int number)
{
        return (number * (number + 1) * ((2* number) + 1)) / 6;
}

// (n*(n+1))/2 specific case of the Faulhaber's formula
unsigned int square_of_sum(unsigned int number)
{
        unsigned int n = (number * (number + 1)) / 2;
        return n * n;
}

unsigned int difference_of_squares(unsigned int number)
{
        return square_of_sum(number) - sum_of_squares(number);
}
