#include "pangram.h"
#include <string>
#include <vector>
#include <cstdint>

namespace pangram {
    bool is_pangram(std::string const&s)
    {
        std::vector <uint8_t>v(128, 0);
        for (int c : s){
            v[c] = 1;
        }
        std::size_t j = 65;
        std::size_t k = 97;
        for (int i = 0; i < 26; i++, j++, k++) {
            if (!(v[j] || v[k])) return false;
        }
        return true;
    }
}
