#if !defined(SECRET_HANDSHAKE_H)
#define SECRET_HANDSHAKE_H
#include <vector>
#include <string>
#include <cstdint>
using namespace std;
namespace secret_handshake {
enum class Action {
    Wink = 1,
    DbBlink = 2,
    ClEyes = 4,
    Jump = 8,
    Reverse = 16
};
vector<string> commands(uint32_t);
}  // namespace secret_handshake

#endif // SECRET_HANDSHAKE_H
