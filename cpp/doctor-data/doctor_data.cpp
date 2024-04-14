#include "doctor_data.h"

void heaven::Vessel::make_buster()
{
    busters += 1; 
}

bool heaven::Vessel::shoot_buster()
{
    if (busters > 0) {
        busters = 0;
        return true;
    }
    return false;
}

heaven::Vessel heaven::Vessel::replicate(std::string name)
{
return Vessel(name, this->generation + 1, this->current_system);
}


std::string heaven::get_older_bob (Vessel &a, Vessel &b)
{
    return a.generation < b.generation ? a.name : b.name;
}


bool heaven::in_the_same_system(Vessel &a, Vessel &b)
{
    return  a.current_system == b.current_system;
}
