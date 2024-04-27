#include <iostream>
#include <sstream>
#include "raindrops.h"

namespace raindrops {

    std::string convert(int i)
    {
        std::stringstream ss;
        bool have = false;
            if ((i % 3) == 0){
                 ss << "Pling";
                 have = true;
            }

            if ((i % 5) == 0) {
                 ss << "Plang";
                 have = true;
            }

            if ((i % 7) == 0) {
                 ss << "Plong";
                 have = true;
            }

            if (!(have)) {
                ss << i;
            }
        return ss.str();
    }
}
