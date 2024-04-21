#include "space_age.h"

namespace space_age {

space_age age(long a)
{
        return space_age (a);                
}

        double space_age::on_mercury () const
        {
            return on_earth() / mercury;
        }
        double space_age::on_venus() const
        {
            return on_earth() / venus;
        }
        double space_age::on_mars() const
        {
            return on_earth() / mars;
        }
        double space_age::on_jupiter() const
        {
            return on_earth() / jupiter;
        }
        double space_age::on_saturn() const
        {
            return on_earth() / saturn;
        }
        double space_age::on_uranus() const
        {
            return on_earth() / uranus;
        }
        double space_age::on_neptune() const
        {
            return on_earth() / neptune;
        }

}
