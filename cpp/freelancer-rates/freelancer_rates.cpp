#include <cmath>

const double DAILY_HOURS {8.0};
const double MONTLY_DAYS {22.0};

double daily_rate(double hourly_rate) {
    return DAILY_HOURS * hourly_rate;
}

double apply_discount(double before_discount, double discount) {
        return before_discount - (before_discount * (discount / 100));
}

int monthly_rate(double hourly_rate, double discount) {
    double dr = hourly_rate * DAILY_HOURS;
    double mr = MONTLY_DAYS * dr;
    double mr_d = mr - (mr * (discount / 100));
    return std::ceil(mr_d);
}

int days_in_budget(int budget, double hourly_rate, double discount) {
    double res = budget / (DAILY_HOURS * apply_discount(hourly_rate, discount));
    return std::floor(res);
}
