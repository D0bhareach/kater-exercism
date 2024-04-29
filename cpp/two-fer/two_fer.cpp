#include "two_fer.h"
#include <sstream>

namespace two_fer
{
    std::string two_fer(const std::string &n)
    {
        std::stringstream ss;
        ss << "One for ";
        ss << n;
        ss << ", one for me.";
        return ss.str();
    }
} // namespace two_fer

