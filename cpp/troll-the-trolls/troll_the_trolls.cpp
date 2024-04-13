namespace hellmath {

enum AccountStatus
{
    troll,
    guest,
    user,
    mod
};

enum Action
{
    read,
    write,
    remove
};

bool display_post(AccountStatus self, AccountStatus other)
{
    bool res = false;
    switch (other){
        case AccountStatus::troll:
                res = true;
            break;
        case AccountStatus::guest:
        case AccountStatus::user:
        case AccountStatus::mod:
            res = (self == AccountStatus::user || self == AccountStatus::mod);
            break;
    }
    return res;
}

bool permission_check(Action action, AccountStatus status)
{
    bool res = false;
    switch (status){
        case AccountStatus::troll:
        case AccountStatus::user:
            res = (action == Action::write || action == Action::read);
            break;
        case AccountStatus::guest:
            res = (action == Action::read);
            break;
        case AccountStatus::mod:
            res = true;
            break;
    }
    return res;
}

bool valid_player_combination(AccountStatus self, AccountStatus other)
{
    bool res = false;
    switch (self){
        case AccountStatus::troll:
            if (other != AccountStatus::troll){
                break;
            } else {
                res = true;
            }
            break;
        case AccountStatus::user:
        case AccountStatus::mod:
            if (other == AccountStatus::user || other == AccountStatus::mod){
                res = true;
            } else {
                break;
            }
            break;
        case AccountStatus::guest:
            if (other == AccountStatus::guest){
                break;
            }
            break;
    }
    return res;
}

bool has_priority(AccountStatus self, AccountStatus other)
{
    bool res = false;
    switch (self){
        case AccountStatus::troll:
            break;
        case AccountStatus::user:
            if (other == AccountStatus::troll || other == AccountStatus::guest)
                res = true;
            break;
        case AccountStatus::guest:
            if (other == AccountStatus::troll)
                res = true;
            break;
        case AccountStatus::mod:
            if (other != AccountStatus::mod)
                res = true;
            break;
    }
    return res;
}


}  // namespace hellmath
