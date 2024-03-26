#include "hamming.h"
#include <string.h>

int compute(const char *lhs, const char *rhs){
        size_t l_len = 0;
        size_t r_len = 0;
        while (lhs[l_len] != '\0') l_len++;
        while (rhs[r_len] != '\0') r_len++;
        if (l_len == r_len && l_len == 0) return 0;
        else if (l_len != r_len) return -1;
        int res = 0;
        for (size_t i = 0; i < l_len; i++) {
                if (lhs[i] != rhs[i]) res++;
        }

        return res;
}
