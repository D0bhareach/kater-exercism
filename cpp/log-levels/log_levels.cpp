#include <string>

namespace log_line {
    std::string message(std::string line) {
        size_t idx = 0;
        idx = line.find(" ", 0);
        return line.substr(++idx);
    }

    std::string log_level(std::string line) {
        size_t idx = 0;
        idx = line.find("]", 0);
        return line.substr(1, --idx);
    }

    std::string reformat(std::string line) {
        size_t idx = 0;
        idx = line.find("]", 0);
        std::string msg = line.substr(idx + 3);
        std::string info = line.substr(1, (idx - 1));
        return msg + " (" + info + ")";
    }
}
