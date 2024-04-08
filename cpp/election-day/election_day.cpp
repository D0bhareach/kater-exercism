#include <string>
#include <vector>

namespace election {

struct ElectionResult {
    std::string name{};
    int votes{};
};

int vote_count(ElectionResult &er){
        return er.votes;
};

void increment_vote_count(ElectionResult &er, int c) {
    er.votes += c;
};


ElectionResult& determine_result(std::vector<ElectionResult> &v){
    if (v.size() == 0) return v[0];
    int president{0};
    for (int i = 1; i < v.size(); i++) {
       if (v[i].votes > v[president].votes) {
            president = i;
               } 
    }
    ElectionResult result = v[president];
    std::string n = result.name;
    v[president].name = "President " + n;
    return v[president];
}

}
