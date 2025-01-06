use std::collections::{HashMap, HashSet};



pub fn solve_one(input:String) -> i32 { 
    let (orders,updates) = input.split_once("\r\n\r\n\r\n").unwrap();

    let orderer = |mut acc : HashMap<String,Vec<String>>,l:&str| {
        let (before,after) = l.split_once("|").unwrap();

        match acc.get_mut(after) {
            Some(vec) => vec.push(before.to_string()),
            None => {
                acc.insert(after.to_owned(), vec![before.to_string()] );
            },
        }
        acc
    };

    let full_rules  = orders.lines().fold(HashMap::new(),orderer);

    updates.lines().filter(|x| is_correct(full_rules.clone(),x)).map(get_middle).sum()
    
}

pub fn is_correct(mut rules: HashMap<String,Vec<String>>,line: &str) -> bool {
    let vals = line.split(",");
    fn consume_number(rules: & mut HashMap<String,Vec<String>>,val: &str, unusable :&mut HashSet<String>) -> bool {
        let found = rules.get(val);
        if unusable.get(val).is_some() {
            return false;
        }
        if found.is_none() {
            return true;
        } 
        found.unwrap().iter().for_each(|b|{ unusable.insert(b.to_string()); });
        true
    };
    
    
    vals.fold((true,HashSet::new()), |(b,mut unusable),val| (b && consume_number(&mut rules,val,&mut unusable),unusable)).0
}

pub fn get_middle(line: &str) -> i32 {
    let vals = line.split(",");
    let size = vals.clone().count();
    vals.skip(size/2).next().unwrap().parse().unwrap()
}


pub fn solve_two(input:String) -> i32 { 
    let (orders,updates) = input.split_once("\r\n\r\n\r\n").unwrap();

    let orderer = |mut acc : HashMap<String,Vec<String>>,l:&str| {
        let (before,after) = l.split_once("|").unwrap();

        match acc.get_mut(after) {
            Some(vec) => vec.push(before.to_string()),
            None => {
                acc.insert(after.to_owned(), vec![before.to_string()] );
            },
        }
        acc
    };

    let full_rules  = orders.lines().fold(HashMap::new(),orderer);

    updates.lines().filter(|x| !is_correct(full_rules.clone(),x)).map(reorder).map(get_middle).sum()
    
}


pub fn reorder(line: &str) -> &str {
    let vals :Vec<&str> = line.split(",").collect();
    let vals_back = vals.clone().reverse();

"a"
}