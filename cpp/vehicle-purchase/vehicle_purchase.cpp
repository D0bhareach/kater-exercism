#include "vehicle_purchase.h"

    // bool needs_license(std::string kind);
    // std::string choose_vehicle(std::string option1, std::string option2);
    // double calculate_resell_price(double original_price, double age);
namespace vehicle_purchase {

    // needs_license determines whether a license is needed to drive a type of vehicle. Only "car" and "truck" require a license.
    bool needs_license(std::string kind){
        return (kind == "truck" || kind == "car");
    }

    // choose_vehicle recommends a vehicle for selection. It always recommends the vehicle that comes first in lexicographical order.
    std::string choose_vehicle(std::string o1, std::string o2) {
        std::string v = o1 < o2 ? o1 : o2;
        return v + " is clearly the better choice.";
    }

    /*
     *if the vehicle is less than 3 years old, it costs 80% of the original
     price it had when it was brand new. If it is at least 10 years old,
     it costs 50%. If the vehicle is at least 3 years old but less than 10
     years, it costs 70% of the original price.
     * */
    double calculate_resell_price(double p, double a) {
        return  (a < 3.0) ? p * 0.8 :
                (a >= 10.0) ? p * 0.5 :
                (a > 3.0 && a < 10.0) ? p * 0.7 : p;
        
    }

}  // namespace vehicle_purchase
