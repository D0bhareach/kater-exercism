#include <array>
#include <string>
#include <vector>
#include <algorithm>
#include <sstream>

std::vector<int> round_down_scores(std::vector<double> student_scores) {
    std::vector<int> r;
    r.reserve(student_scores.size());
    for (auto n : student_scores) {
        r.push_back(static_cast<int>(n));
    }
    return r;
}


int count_failed_students(std::vector<int> student_scores) {
    int res{0};
    for (int s : student_scores) {
        if (s < 41) res++; 
    }
    return res;
}

std::vector<int> above_threshold(std::vector<int> student_scores, int threshold) {
    std::vector<int> res;
    res.reserve(student_scores.size()); 
    for (int s : student_scores) {
        if (s >= threshold) res.push_back(s);
    }
    return res;
}

std::array<int, 4> letter_grades(int highest_score) {
    std::array<int, 4> res;
    int grade{41};
    int inc;
    inc = (highest_score - 40) / 4;
    res[0] = grade;
    for (int i = 1; i < 4; i++) {
        grade += inc;
        res[i] = grade;
        
    }
    return res;
}

std::vector<std::string> student_ranking(
        std::vector<int> student_scores,
        std::vector<std::string> student_names) {
    int idx{0};
    auto do_string = [idx](std::string &name, int score) mutable -> std::string{
                   idx++;
                   std::stringstream sstm;
                   sstm << idx << ". " << name << ": " << score;
                   return sstm.str();
                   };

    std::transform(
            student_names.begin(), student_names.end(), student_scores.begin(),
            student_names.begin(), do_string);
    return student_names;
}

std::string perfect_score(
        std::vector<int> student_scores, std::vector<std::string> student_names) {
    auto it = std::find_if(student_scores.begin(), student_scores.end(),
            [](int s) {
                return s == 100;
            });
    if (it != student_scores.end()) {
        int index = std::distance(student_scores.begin(), it);
        return student_names[index];
    }
    return "";  
}
