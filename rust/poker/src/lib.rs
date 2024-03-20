/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
use std::cmp::{Ordering, PartialOrd};
use std::fmt::Debug;
use std::cmp::{Eq, PartialEq};
// https://blog.logrocket.com/understanding-rust-generics/#lifetime-generics-rust
pub struct PokerHand<'a>(&'a str);
/*
NOTES!!!: For now idea is to sort hand create PokerHandInner to find out what category hand
belongs to.
For this I need to find out if all suites are the same and if they are must
find out are they sequetial or not.
Than I need to find out if there are same ranks to find pair, triplets ect. May need to use
vectors of the same ranks in or indexes of the same ranks. For now it simply possible to
see if I have one rank. See tests below.
Whole approach is somehow ugly. Must find algorithm for hands and categories.
Must prioritize categries. One is bigger than other.
SEARCH FOR BETTER ALGORITHM!!!
*/

#[derive(PartialEq, Debug)]
struct PockerHandInner<'a>{
    hearts: u8,
    spade: u8,
    clubs: u8,
    diamonds: u8,
    rank_holder_1: char,
    rank_holder_2: char,
    rank_1: u8,
    rank_2: u8,
    hand: &'a str,
}

impl<'a> PockerHandInner<'a> {
    fn new(s: &'a str) -> Self {
        PockerHandInner {
            hearts: 0,
            spade: 0,
            clubs: 0,
            diamonds: 0,
            rank_holder_1: char::default(),
            rank_holder_2: char::default(),
            rank_1: 0,
            rank_2: 0,
            hand: s
        }
    }
}
/*
fn category(s: &str) -> PockerHandInner {
    s.split(" ").fold(PockerHandInner::new(s), |p, s| {
        let rank = s.chars().nth(0).unwrap();
        let suite = s.chars().nth(1).unwrap();
        match suite {
            'C' => p.clubs += 1,
            'D' => p.diamonds += 1,
            'H' => p.hearts += 1,
            'S' => p.spade += 1,
        }
        if p.rank_holder_1.eq(&char::default()) {
            p.rank_holder_1 = rank;
            p.rank_1 += 1;
        } else if p.rank_holder_1.is_uppercase() && p.rank_holder_1.eq(&rank){
            p.rank_1 += 1;
        } else if p.rank_holder_1.ne(&rank) {
            // check if rank 2 is empty if empty assign
            if p.rank_holder_2.eq(&char::default()){
                p.rank_holder_2 = rank;
                p.rank_2 += 1;
            } else if p.rank_holder_2.eq(&rank){
                p.rank_2 += 1;
            } else {
                #[allow(clippy::collapsible_else_if)]
                if p.rank_1.lt(&2){
                    p.rank_holder_1 = rank;
                    p.rank_1 = 1;
                } else if p.rank_2.lt(&2){
                    p.rank_holder_2 = rank;
                    p.rank_2 = 1;
                }
            }
        }

        p
    })
}
*/

fn same_suites(v: &Vec<&str>) -> bool {
    let mut c = char::default();
    for s in v.iter() {
        let suite_char = s.chars().last().expect("unable to fetch suite from string slit");
        if c.eq(&char::default()){
            c = suite_char;
        } else if suite_char != c{
            return false;
        }
    }
        true
}
impl<'a> From<PokerHand<'a>> for PockerHandInner<'a> {
    fn from(h: PokerHand<'a>) -> Self {
    let mut s = h.0.split(' ').collect::<Vec<&str>>();
    s.sort();
    s.iter().fold(PockerHandInner::new(h.0), |mut p, s| {
        let mut s_iter = s.chars();
        let rank = s_iter.next().unwrap();
        let suite = s_iter.next().unwrap();
        match suite {
            'C' => p.clubs += 1,
            'D' => p.diamonds += 1,
            'H' => p.hearts += 1,
            'S' => p.spade += 1,
            _ => (),
        }
        if p.rank_holder_1.eq(&char::default()) {
            p.rank_holder_1 = rank;
            p.rank_1 += 1;
        } else if p.rank_holder_1.is_uppercase() && p.rank_holder_1.eq(&rank){
            p.rank_1 += 1;
        } else if p.rank_holder_1.ne(&rank) {
            // check if rank 2 is empty if empty assign
            if p.rank_holder_2.eq(&char::default()){
                p.rank_holder_2 = rank;
                p.rank_2 += 1;
            } else if p.rank_holder_2.eq(&rank){
                p.rank_2 += 1;
            } else {
                #[allow(clippy::collapsible_else_if)]
                if p.rank_1.lt(&2){
                    p.rank_holder_1 = rank;
                    p.rank_1 = 1;
                } else if p.rank_2.lt(&2){
                    p.rank_holder_2 = rank;
                    p.rank_2 = 1;
                }
            }
        }

        p
    })
    }
}

/*
impl<'a>  PartialOrd for PokerHand<'a>
// where T: &'a String,
{
    fn partial_cmp(&self, other: &PokerHand<'a>) -> Option<Ordering> {
        // step 3?
        let indexes = [12, 0, 3, 6, 9];
        // use try_fold
        let mut res = Some(Ordering::Equal);
        for i in indexes {
            let o = self.0.get(i).unwrap().cmp(other.0.get(i).unwrap());
            match o {
                Ordering::Less => {
                    res = Some(Ordering::Less); break;},
                Ordering::Greater => {
                    res = Some(Ordering::Greater); break;},
                Ordering::Equal =>continue,
            }
            res
        }
    }
    
}
*/
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    //let hs: Vec<&str> = *hands.split(',').collect();
    let r: Vec<&str> = hands.to_vec();
    r
}

/*
Need iterator for PockerHand, (than it must be sorted?), than it must be categorized.
Looks like I need to get anoter tuple sruct Card(u8,u8) first is suite, second is rank/number
than I maight to test for categorie pattern. It's not clear on how to test between categories
yet.

Categories:
Five of a kind - if hand has a jocker and other 4 are of the same rank(aces, kings, queens, )
There is no cases in this app for this kind of hand

Straight flush - is a hand that contains five cards of sequential rank, all of the same suit.
There is special condition on how to count Ace. If had contains pictures it count as highes
if no pictures as lowest. Basically check if hand is all the same suite, if it contains ace
change weight of ase Card.0 = 1.
can try to iterator all to see if the same suite
{

    cards of same_suite_1 = 0; // need suite holder
    cards of same_suite_2 = 0; // need suite holder
    cards_of_same_suite_3 = 0;

    four_of_rank // count rank if the same
    three_of_rank // need holder of two ranks.
    two_of_rank_1 //
    two_of_rank_2 //
}

Four of a kind - is a hand that contains four cards of one rank and one card of another
rank (the kicker). Check Card.0 if it all the same.

Full house - is a hand that contains three cards of one rank and two cards of another rank.

Flush - contains five cards all of the same suit, not all of sequential rank.

Straight - contains five cards of sequential rank, not all of the same suit

Three of a kind -  a hand that contains three cards of one rank and two cards of two other
ranks (the kickers)

Two pair - contains two cards of one rank, two cards of another rank and one card of
a third rank (the kicker),

One pair - s a hand that contains two cards of one rank and three cards of three other
ranks (the kickers), 

High card -  no pair or simply nothing, is a hand that does not fall into any other category

*/

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    #[ignore]
    fn test_from(){
        let act = PockerHandInner::from(PokerHand("2S 4H 6S 4D JH"));
        let exp = PockerHandInner::new("2S 4H 6S 4D JH");
        assert_eq!(act, exp);
    }

    #[test]
    fn test_same_suites(){
        let hands = vec!["2H 3H 4H 5H 6H", "KH QH JH 10H 9H", "2S 5S JS AS 4S", "10S 4S 5S 8S AS"];
        for h in hands.iter() {
        let mut s = h.split(' ').collect::<Vec<&str>>();
        s.sort();
        assert!(same_suites(&s), "{:?}", s);
        };

    }
}