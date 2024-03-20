#include "armstrong_numbers.h"
#include <stdio.h>
#include <math.h>


const int LEN = 20;
int get_chars_len(char * chars);
     
bool is_armstrong_number(int candidate)
{
    char chars[20];
    int chars_len = 1;
    int sum = 0;
    int candidate_origin = candidate;
    for (int i = 0; i < LEN; i++)
    {
        chars[i] = 'x';
    }
    snprintf(chars, LEN, "%d", candidate);
    chars_len = get_chars_len(chars);
    for (int i = 0; i < chars_len; i++)
    {
        int tmp = 0;
        int p = ((chars_len - 1)  - i);
        int n = candidate / pow(10, p);
        tmp = n;
        n = pow(n, chars_len);
        sum += n;
        candidate = candidate - (tmp * pow(10, p));
    }
    return sum == candidate_origin; 
}

int get_chars_len(char * chars)
{
    int res = 0;
    for (int i = 0; i < LEN; i++)
    {
        char c = chars[i];
        if(c == '\0' || c == 'x')
        {
            continue;
        }
            res++;
    }
    return res;
}
