#if !defined(TRIANGLE_H)
#define TRIANGLE_H

namespace triangle {
        enum class flavor {
        equilateral,
        isosceles,
        scalene
        };

        flavor get_kind(double, double, double);


        template <typename T>
        flavor kind(T a, T b, T c)
    {
        double x = static_cast<double>(a);
        double y = static_cast<double>(b);
        double z = static_cast<double>(c);
        return get_kind(x, y, z);

    }

        template <typename U, typename T>
    flavor kind(U a, T b, T c)
    {
        double x = static_cast<double>(a);
        double y = static_cast<double>(b);
        double z = static_cast<double>(c);
        return get_kind(x, y, z);

    }


}  // namespace triangle

#endif // TRIANGLE_H
