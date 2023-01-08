// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
use std::collections::HashMap;
pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut map: HashMap<&str, i32> = HashMap::new();
    let mut can_construct: bool = true;
    for word in magazine {
        *map.entry(word).or_insert(1) += 1;
    }
    for word in note {
        *map.entry(word).or_insert(0) -=1;
        if(map[word] < 1) {
            can_construct = false;
        }
    }
    can_construct
}