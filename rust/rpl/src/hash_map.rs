use std::collections::HashMap;

#[allow(dead_code)]
pub fn hash_map_main(){
    let mut map = HashMap::new();
    map.insert(String::from("blue"), (10,10,10));

    let teams = vec![String::from("blue"), String::from("red")];
    let scores = vec![10,20];
    let mut map : HashMap<_,_> = teams.iter().zip(
        scores.iter()
    ).collect();

    println!("{:?}", map);
    let key = String::from("blue");
    println!("{:?}", map.get(&key));

    for (k,v) in map {
        println!("{} {}", k , v);
    }

    let mut map = HashMap::new();
    map.entry(String::from("white")).or_insert(10);
}