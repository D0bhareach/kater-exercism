#include "grains.h"
#include <math.h>

uint64_t square(uint8_t index)
{
        if (index == 0 || index ==1) return index;
        else if (index > 64) return 0;
        return 1 * powl(2, (index - 1));
}

uint64_t total(void)
{
        uint64_t res = 0;
        for (int i = 1; i < 65; i++){
                res += square(i);
        }
        return res;
}
