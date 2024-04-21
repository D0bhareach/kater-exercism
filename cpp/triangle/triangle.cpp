#include "triangle.h"
#include <iostream>
#include <stdexcept>

namespace triangle {

    std::string check_parameters(double x, double y, double z)
    {
        std::string res {""};
        if (x == 0 && y == 0 && z == 0)
            res = "triangles with no size";
        else if (x < 0 || y < 0 || z < 0)
            res = "triangles with negative side";
        else if (!((x + y) >= z && (x + z) >= y && (y + z) >= x))
            res =  "violating triangle inequality";
        return res;
    }
    
    flavor get_kind(double x, double y, double z){
        std::string err = check_parameters(x, y, z);
        if (err.size() > 0) throw std::domain_error(err);
        if (x == y && x == z) return triangle::flavor::equilateral;
        else if (x == y || x == z || y == z) return triangle::flavor::isosceles;
        else if (x != y && x != z && y != z) return triangle::flavor::scalene;
        std::string msg = "no conditions met";
        throw std::domain_error(msg);
    }

}
