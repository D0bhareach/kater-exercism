#include "secret_handshake.h"
#include <iostream>
#include <bitset>
#include <algorithm>

using namespace std;
namespace secret_handshake {

// want to try how to use enums in c++
vector<string> commands(uint32_t n)
{
    vector<string> res;
    res.reserve(5);
    
    if(n == 0) return res;
    vector<Action> act;
    act.reserve(5);

    bitset<sizeof(int) * 8> bits(n);    
     for (int i = bits.size() -1; i > -1;  i--) {
        switch (i) {
            case 0:
                if (bits[i]) act.push_back(Action::Wink);
                break;
            case 1:
                if (bits[i]) act.push_back(Action::DbBlink);
                break;
            case 2:
                if (bits[i]) act.push_back(Action::ClEyes);
                break;
            case 3:
                if (bits[i]) act.push_back(Action::Jump );
                break;
            case 4:
                if (bits[i]) act.push_back(Action::Reverse );
                break;
        }
     }
        // now that we have actions must create strings
         for (auto rit = act.rbegin(); rit != act.rend(); rit++) {
             switch (*rit) {
                 case Action::Wink:
                    res.push_back("wink");
                    break;
                 case Action::DbBlink:
                    res.push_back("double blink");
                    break;
                 case Action::ClEyes:
                    res.push_back("close your eyes");
                    break;
                 case Action::Jump:
                    res.push_back("jump");
                    break;
                 case Action::Reverse:
                    res.shrink_to_fit();
                    if (res.size() > 1)
                        reverse(res.begin(), res.end());
                    break;

             }
        }

        return res;
    }
}
