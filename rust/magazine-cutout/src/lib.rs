use std::collections::HashMap;

/*
use std::collections::hash_map::Entry;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut res = true;
    let mut map: HashMap<&str, usize> = HashMap::with_capacity(magazine.len());
    magazine.iter().for_each(|w|{ 
        map.entry(w).and_modify(|c| *c += 1).or_insert(1);
    });

    for w in note.iter() {
        match map.entry(w) {
            Entry::Occupied(mut e) => {
                let v = e.get();
                if v.eq(&1){
                    e.remove_entry();
                } else if v.gt(&1){
                    e.insert( v-1 );
                } else { 
                    res = false;
                    break;
                }},
            Entry::Vacant(_) => {
                res = false;
                break;
            }
        }
    }
    res
}
*/

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut res = true;
    let mut map: HashMap<&str, usize> = HashMap::with_capacity(magazine.len());
    magazine.iter().for_each(|w|{ 
        map.entry(w).and_modify(|c| *c += 1).or_insert(1);
    });

    for w in note.iter() {
        if let Some(c) = map.get(w) {
            if c.ge(&1) {
                let nc = c - 1;
                map.insert(w, nc);
            } else {
                res = false;
                break;
            };
        } else {
            res = false;
            break;
        }
    }
    res
}