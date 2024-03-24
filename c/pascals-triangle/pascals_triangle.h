#ifndef PASCALS_TRIANGLE_H
#define PASCALS_TRIANGLE_H

#include <stddef.h>
#include <stdint.h>

void free_triangle(uint8_t **triangle, size_t rows);
uint8_t **create_triangle(size_t rows);
// my util functions
int get_factorial(uint8_t n);
uint8_t * create_row(uint8_t n, size_t size);
void print_arr(uint8_t * arr, size_t size);

#endif
