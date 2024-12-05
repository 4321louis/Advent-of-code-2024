pub fn solve_one(input:String) -> i32 {
    let mut acc = 0;
    let mut i=0;
    while i < input.len() {
        let rest = &input[i..];
        match parse_first_mul(rest) {
            Some(n) => {
                acc += n;
                i += rest.chars().position(|c| c == ')').unwrap()
            },
            
            None => i+=1,
        }
    }
    acc
}

fn parse_first_mul(rest:&str) -> Option<i32> {
    let rest = eat_chars(rest, "mul(")?;
    let (n1,l) = parse_number(rest)?;
    if l>3 {
        return None;
    }
    let rest = &rest[l..];
    let rest = eat_chars(rest, ",")?;

    let (n2,l) = parse_number(rest)?;
    if l>3 {
        return None;
    }
    let rest = &rest[l..];
    let rest = eat_chars(rest, ")")?;

    Some(n1*n2)
}
fn eat_chars<'a>(rest: &'a str,eat :&'a str) -> Option<&'a str> {
    
    if !rest.starts_with(eat) {
        return None
    }
    Some(&rest[eat.len()..])

}
fn parse_number(rest:&str) -> Option<(i32,usize)> {
    let nonNumber = rest.chars().position(|c| !c.is_numeric()).unwrap_or(rest.len());
    Some((rest[..nonNumber].parse().ok()?,nonNumber))
}
pub fn solve_two(input:String) -> i32 {
    let bits = get_doed(&input);
    bits.iter().map(|b| solve_one(b.to_string())).fold(0, |acc,n|acc+n)
}

fn get_doed(input: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = input;
    while let Some(i) = rest.find("don't()") {
        out.push(rest[..i].to_string());
        rest = &rest[i+7..];
        match rest.find("do()") {
            Some(i) => {
                rest = &rest[i..];
            },
            None => {return out},
        }
    }
    out.push(rest.to_string());
    out

}
