#ifndef DOCTOR_DATA
#define DOCTOR_DATA

#include <string>
namespace star_map {

    enum class System {
        Sol,
        EpsilonEridani,
        AlphaCentauri,
        DeltaEridani,
        Omicron2Eridani,
        BetaHydri
    };

}

namespace heaven {
    using namespace star_map;
    class Vessel {
        public:
            std::string name;
            int generation;
            star_map::System current_system {star_map::System::Sol};
            int busters {0};
            Vessel(std::string n, int g):name{n}, generation{g}{
                current_system = star_map::System::Sol;
            };

            Vessel(std::string n, int g, star_map::System s): \
                name{n}, generation{g}, current_system{s}{};

            void make_buster();
            bool shoot_buster();
            Vessel replicate(std::string name);
    };
    std::string get_older_bob (Vessel &a, Vessel &b);
    bool in_the_same_system(Vessel &a, Vessel &b);
}


#endif
