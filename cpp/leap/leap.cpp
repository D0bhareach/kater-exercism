#include "leap.h"

namespace leap {
bool is_leap_year(int y)
{
return y % 4 == 0 ? y % 100 == 0 ? y % 400 == 0 : true : false;
}
}  // namespace leap
