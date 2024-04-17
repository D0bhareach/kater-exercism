#include "lasagna_master.h"
#include <string>
#include <vector>

using namespace std;
namespace lasagna_master {

int preparationTime(const std::vector<std::string> &layers, int time)
{
    std::size_t size = layers.size();
    return static_cast<int> (size * time);
}


amount quantities(const std::vector<std::string> &layers) {
    amount res;
    int n {0};
    double s {0.0};

    for (std::string ingr : layers){
        if (ingr == "noodles") {
            n += 50;
        } else if (ingr == "sauce") {
            s += 0.2;
        }
    }

    res.noodles = n;
    res.sauce = s;
    return res;
}

void addSecretIngredient( \
        std::vector<std::string> &myList,\
        const std::vector<std::string> &friendsList)
{
    if(!friendsList.empty()){
        std::size_t len = myList.size();
        std::size_t idx = len > 0 ? len -1 : 0;
        myList.at(idx) = friendsList.back();
    }
}

void addSecretIngredient(\
        std::vector<std::string> &myList, const std::string secretIngredient)
{
        myList.pop_back();
        myList.push_back(secretIngredient);
}

std::vector<double> scaleRecipe(const std::vector<double> &quantities, int s)
{
    std::vector<double> res;
    double sc = s / 2.0;
    for (double d : quantities){
        res.push_back(d * sc);
    }
    return res;
}


}
