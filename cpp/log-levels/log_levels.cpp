#include <string>
//
#include <iostream>

namespace log_line {
    std::string message(std::string line) {
        size_t idx = line.find(":");
        std::cout << "idx: " <<idx<<std::endl;
        return line;
        // return the message
    }

    std::string log_level(std::string line) {
        return line;
        // return the log level
    }

    std::string reformat(std::string line) {
        return line;
        // return the reformatted message
    }
}
