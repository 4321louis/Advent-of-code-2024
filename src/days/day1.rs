
use std::collections::HashMap;

pub fn solve_one(input:String) -> i32 {
    let lines = input.lines();
    let mut list1 = Vec::<i32>::new();
    let mut list2 = Vec::<i32>::new();
    for line in lines {
        let mut numbers = line.split("   ");
        list1.push(numbers.next().unwrap().parse().unwrap());
        list2.push(numbers.next().unwrap().parse().unwrap());
    }
    list1.sort();
    list2.sort();
    list1.iter().zip(list2).map(|(n1,n2)| (n1 - n2).abs() ).fold(0,|acc,x| acc + x)
}

pub fn solve_two(input :String) -> i32 {
    let lines = input.lines();
    let mut list1 = Vec::<i32>::new();
    let mut list2 = Vec::<i32>::new();
    for line in lines {
        let mut numbers = line.split("   ");
        list1.push(numbers.next().unwrap().parse().unwrap());
        list2.push(numbers.next().unwrap().parse().unwrap());
    }
    
    let mut calced_scores = HashMap::<i32,i32>::new();
    let mut acc = 0;
    for n in list1 {
        acc += match calced_scores.get(&n) {
            Some(x) => *x,
            None => {
                let score = n*list2.iter().filter(|x| **x==n).count() as i32;
                calced_scores.insert(n, score);
                score
            }
        }
    }
    acc
}