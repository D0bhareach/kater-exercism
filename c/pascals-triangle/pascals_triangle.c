#include "pascals_triangle.h"
#include <stdint.h>
#include <stdlib.h>
// hide below
// #include <stdio.h>

/*
void print_arr(uint8_t * arr, size_t size)
{
    printf("{ ");
    for (size_t i = 0; i < size ; i++)
    {
        printf("%d, ", arr[i]);
    }
    printf("}\n");
}
*/
int get_factorial(uint8_t n)
{
    int res = 1;
    uint8_t x = 1;
    while (x <= n)
    {
        res *= x;
        x++;
    }
    return res;
}

// n - number of row
uint8_t * create_row(uint8_t n, size_t size)
{
    uint8_t * arr = NULL;
    arr = malloc(size * sizeof(uint8_t));

    // assign all zeros to result array
    for (size_t i = 0; i < size; i++)
    {
        arr[i] = 0;
    }

    uint32_t f_n = 0;
    
    uint8_t tmp = 1;
    if(n < 2)
    {
    tmp = 1;
    }
    else
    {
        tmp = n;
    }
    f_n = get_factorial(tmp);

    for (size_t i = 0; i <= n ; i++)
    {
        // first element is always = 1
        if (i == 0)
        {
            arr[i] = 1;
            continue;
        }
        uint8_t e = 0;
        uint32_t f_k = get_factorial(i);
        uint32_t right = get_factorial(tmp - i);
        e = (uint8_t)(f_n / (f_k * right));
        arr[i] = e;
    }
    return arr; 
}

void free_triangle(uint8_t **triangle, size_t rows)
{
    for (size_t i = 0; i < rows ; i++)
    {
        free(triangle[i]);
        triangle[i] = NULL; // is it worth it?
    }
    free(triangle);
    triangle = NULL;
}

uint8_t **create_triangle(size_t rows)
{
    // initialize result array
    uint8_t ** arr = NULL;
    size_t res_rows = 1;
    if (rows > 1)
    {
        res_rows = rows;
    }
    arr = malloc(res_rows * sizeof(uint8_t *)); 


    if (rows == 0)
    {
        arr[0] = malloc(1 * sizeof(uint8_t));
        arr[0][0] = 0;
    }
    else if (rows == 1)
    {
        arr[0] = malloc(1 * sizeof(uint8_t));
        arr[0][0] = 1;
    }
    else
    {
        for (size_t i = 0; i  < res_rows; i++)
        {
            uint8_t * row  = create_row(i, res_rows);
            arr[i] = row;
        }

    }

    return arr; 
}
