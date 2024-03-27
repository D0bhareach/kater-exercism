#include "square_root.h"
#include <math.h>

unsigned int square_root(unsigned int num)
{
        if (num == 1) return num;
        float prev = 1.0;
        float r = 1.0;
        float epsylon = 0.1;
        while (1 == 1) {
                prev = r;
                r = 0.5 * (r + (num / r));
                if (fabs(prev -r) < epsylon) return r;
        }
        return r;
}
