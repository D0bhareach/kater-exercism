#if !defined(SPACE_AGE_H)
#define SPACE_AGE_H

namespace space_age {
        enum class orbital_seconds {
        };

        class space_age {
                private:
                        long age_seconds{0};
                        double earth_age{0.0};
                        const double earth = 31557600.0;
                        const double mercury = 0.2408467;
                        const double venus = 0.61519726;
                        const double mars = 1.8808158;
                        const double jupiter = 11.862615;
                        const double saturn = 29.447498;
                        const double uranus = 84.016846;
                        const double neptune = 164.79132;

                public:
                        space_age(long s): age_seconds(s){
                                earth_age = age_seconds / earth;
                        }

                        
                        long seconds() const {
                                return this->age_seconds;
                                }

                        double on_earth() const
                        {
                                return this->earth_age;
                        }
                        double on_mercury () const;
                        double on_venus() const;
                        double on_mars() const;
                        double on_jupiter() const;
                        double on_saturn() const;
                        double on_uranus() const;
                        double on_neptune() const;
                
        };

        space_age age(long a);
}

#endif // SPACE_AGE_H
