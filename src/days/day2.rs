

pub fn solve_one(input:String) -> i32 {
    let lines = input.lines(); 
    let reports :Vec<Vec<i32>> = lines.map(|n| n.split(' ').map(|n| n.parse().unwrap()).collect()).collect();
    reports.iter().filter(|r| is_safe(r).is_none()).count() as i32
    
}
fn is_safe(report :&Vec<i32>) -> Option<usize> {
    let mut difs = report.iter().zip(report.iter().skip(1)).map(|(n1,n2)| n1-n2);
    let first = difs.next().unwrap_or(1);
    if !(first.abs() >= 1 && first.abs() <= 3) {
        return Some(1);
    }
    let test = if first > 0 {
        |dif| !(dif>=1 && dif<=3)
    } else {
        |dif| !(dif<=-1 && dif>=-3)
    };
    difs.position(test).and_then(|n|Some(n+2))
}


pub fn solve_two(input:String) -> i32 {
    let lines = input.lines(); 
    let reports :Vec<Vec<i32>> = lines.map(|n| n.split(' ').map(|n| n.parse().unwrap()).collect()).collect();
    reports.iter().filter(|r| is_safe_two(r.to_vec())).count() as i32
    
}

fn is_safe_two(mut report :Vec<i32>) -> bool {
    match is_safe(&report) {
        Some(i) => {
            (
                i==2 && {
                    let mut report1 = report.clone();
                    report1.remove(i-2);
                    is_safe(&report1).is_none()
                }
            ) || (
                i<=2 && {
                    let mut report1 = report.clone();
                    report1.remove(i-1);
                    is_safe(&report1).is_none()
                }
            ) || {
                report.remove(i);
                is_safe(&report).is_none()
            }
        },
        None => true
    }
}
