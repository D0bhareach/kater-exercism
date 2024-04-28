#include <iostream>
#include <sstream>
#include <map>
#include "raindrops.h"

namespace raindrops {
    std::map<int, std::string> drops = {
            {3, "Pling"},
            {5, "Plang"},
            {7, "Plong"},
    };

    std::string convert(int i)
    {
        std::stringstream ss;
        bool have = false;
        for (auto const& pair:drops) {
            if ((i % pair.first) == 0){
                 ss << pair.second;
                 have = true;
            }
                
        }

        if (!(have)) {
            ss << i;
        }
        return ss.str();
    }
}
