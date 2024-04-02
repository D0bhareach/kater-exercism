#include "binary_search.h"

const int *binary_search(int value, const int *arr, size_t length)
{
        if (length == 0) return NULL;
        size_t last = length - 1;
        if (value <2) {
                if (arr[0] == value) {
                        return arr;
                } else {
                        return NULL;
                }
        } else if (value > arr[last]) {
                return NULL;
        } else if (value == arr[last]) {
                return &arr[last];
        }

        size_t mid = length / 2;
        size_t start = 0;
        size_t end = length;

        while (length != 0) {
                if (value == arr[mid]){
                        return &arr[mid];
                } else if (value < arr[mid]) {
                        length = mid - start;
                        end = --mid;
                        mid = start + (length / 2) ;
                        continue;
                } else if (value > arr[mid]) {
                        start = ++mid;
                        length = end - start;
                        mid = (length / 2) + start;
                        if (start == mid) {
                                if (arr[mid] == value) return &arr[mid];
                        }
                        continue;
                }
        }

        return NULL;
}
