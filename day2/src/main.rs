use std::io;

fn main() {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).unwrap();

    let mut found = false;
    let mut verb = 0;
    let mut noun = 0;

    for x in 0..99 {
        for y in 0..99 {
            let mut codes = &mut parse_input(&inp)[..];
            codes[1] = x;
            codes[2] = y;

            let res = process_codes(&mut codes);
            if res == 19690720 {
                found = true;
                println!("verb: {}", y);
                verb = y;
                break
            }
        }

        if found == true {
            println!("noun: {}", x);
            noun = x;
            break;
        }
    }

    println!("result: {}", 100 * noun + verb);
}

fn parse_input(inp: &str) -> Vec<usize> {
    inp.split(",")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

fn process_codes(codes: &mut [usize]) -> usize {
    let mut i = 0;
    while i < codes.len() {
        let code: usize= codes[i];

        if code != 99 {
            if code == 1 {
                op_1(codes[i + 1], codes[i + 2], codes[i + 3], codes);
            } else if code == 2 {
                op_2(codes[i + 1], codes[i + 2], codes[i + 3], codes);
            }

            i += 4;
        } else {
            break;
        }
    }

    codes[0]
}

fn op_1(x: usize, y: usize, pos: usize, codes: &mut [usize]) {
    if pos < codes.len() {
        let sum = codes[x] + codes[y];
        codes[pos] = sum;
    }
}

fn op_2(x: usize, y: usize, pos: usize, codes: &mut [usize]) {
    if pos < codes.len() {
        let prod = codes[x] * codes[y];
        codes[pos] = prod;
    }
}
