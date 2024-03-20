enum Map<'a> {
    Nail(&'a str),
    Shoe(&'a str),
    Horse(&'a str),
    Rider(&'a str),
    Message(&'a str),
    Battle(&'a str),
    Kingdom(&'a str),
}
fn is_sane(list: &[&str]) -> bool {
    let mut res = false;
    let probe = vec![
        "nail", "shoe", "horse", "rider", "message", "battle", "kingdom",
    ];
    for w in list {
        res = probe.contains(w);
        if res == false {
            return res;
        }
    }
    res
}
pub fn build_proverb(list: &[&str]) -> String {
    // catch special cases
    if list.len() == 0 {
        return String::new();
    }
    if !is_sane(list) {
        return String::from(
            "For want of a pin the gun was lost.
For want of a gun the soldier was lost.
For want of a soldier the battle was lost.
And all for the want of a pin.",
        );
    }
    let mut new_list = list[..].to_vec();
    let length = new_list.len();
    if length == 1 {
        return String::from("And all for the want of a nail.");
    } else if length > 1 && length < 7 {
        new_list[length - 1] = &"kingdom";
    }

    let mut res = String::new();
    for key in new_list {
        match key {
            "nail" => {
                if let Map::Nail(s) = Map::Nail("For want of a nail the shoe was lost.") {
                    res.push_str(s);
                    res.push_str("\n");
                }
            }
            "shoe" => {
                if let Map::Shoe(s) = Map::Shoe("For want of a shoe the horse was lost.") {
                    res.push_str(s);
                    res.push_str("\n");
                }
            }
            "horse" => {
                if let Map::Horse(s) = Map::Horse("For want of a horse the rider was lost.") {
                    res.push_str(s);
                    res.push_str("\n");
                }
            }
            "rider" => {
                if let Map::Rider(s) = Map::Rider("For want of a rider the message was lost.") {
                    res.push_str(s);
                    res.push_str("\n");
                }
            }
            "message" => {
                if let Map::Message(s) = Map::Message("For want of a message the battle was lost.")
                {
                    res.push_str(s);
                    res.push_str("\n");
                }
            }
            "battle" => {
                if let Map::Battle(s) = Map::Battle("For want of a battle the kingdom was lost.") {
                    res.push_str(s);
                    res.push_str("\n");
                }
            }
            "kingdom" => {
                if let Map::Kingdom(s) = Map::Kingdom("And all for the want of a nail.") {
                    res.push_str(s);
                    res.push_str("\n");
                }
            }
            _ => (),
        }
    }
    if let Some(i) = res.rfind('\n') {
        res.remove(i);
    }
    res
}

//
// Firstly I tried to create constant map inside the module it  was unsuccessful.
// Looks like it's not possible with Rust. I guess because of allocations?
// Here is an article that gave me a clue: https://users.rust-lang.org/t/is-there-a-way-to-create-a-constant-map-in-rust/8358/11
// Then solution emerged, some clues I have got from here:
// https://stackoverflow.com/questions/9109872/how-do-you-access-enum-values-in-rust
// Interesting point how to convert slice to mut slice:
// https://stackoverflow.com/questions/24872634/how-do-i-create-two-new-mutable-slices-from-one-slice
// This solution with enum works, but image there will be a few hundred of such beautiful
// lines becase we will have to go through all Enums. So first is more appealing to me.
// I didn't properly understand what exactly was required from me, so basically it's
// TDD. And a fine example of me solving the problem with shite & sticks.
