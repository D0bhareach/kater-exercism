#include "pangram.h"
#include <bitset>
#include <string>
#include <iostream>
#include <vector>
#include <algorithm>
#include <cstdint>

namespace pangram {
    bool is_pangram(std::string const&s)
    {
        std::vector <uint8_t>v(128, 0);
        std::bitset<25> b1;
        std::bitset<25> b2;
        for (int c : s){
            v[c] = 1;
        }

        std::size_t i{0};
        std::for_each(v.begin() + 65, v.begin() + 91, [&](uint8_t val)
            { b1[i] = val; ++i; });

        std::size_t j{0};
        std::for_each(v.begin() + 97, v.begin() + 123, [&](uint8_t val)
            { b2[j] = val; ++j; });

        b1 |= b2;

// bitwise all() always return 0 and counter() is always return one  more
// than actual size()  Why?
        int counter = b1.count() - 1;
        return (bool)(counter == 25);
    }
}
