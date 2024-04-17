#include "reverse_string.h"

namespace reverse_string {
    std::string reverse_string(std::string s)
    {
        if (s.size() == 0) {
            return s;
        }
        std::string res;
        res.reserve(s.size());
        std::copy(s.crbegin(), s.crend(), std::back_inserter(res));
        return res;
    }
}
