use std::collections::HashMap;

pub fn mode(integers: &[i32]) -> i32 {
    let mut hashmap: HashMap<i32, i32> = HashMap::new();

    for number in integers {
        let e = hashmap.entry(*number).or_insert(0);
        *e += 1;
    }

    *hashmap.iter().max_by_key(|val| val.1).unwrap().0
}
