#include "grains.h"
#include <cmath>
#include <cstddef>
#include <string>

namespace grains {

unsigned long long square(int iter)
{
    if (iter > 0 && iter < 4){
        return 1 << (iter - 1);
    }

        long double res = std::pow(2, iter);
        return static_cast<unsigned long long>(res/2);
}
unsigned long long total()
{
unsigned long long res{0};
for (int i = 1; i < 65; i++){
    res += square(i);
}
return res;
}
}
