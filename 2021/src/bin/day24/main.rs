use derive_new::new;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

#[derive(Debug, Clone, Copy)]
struct StepParams {
    do_cmp: bool,
    cmp_now: i8,
    cmp_later: i8,
}

#[allow(clippy::needless_range_loop)]
fn parse(data: &'static str) -> [StepParams; 14] {
    let lines: Vec<_> = data.lines().collect();
    let mut params = [None; 14];
    let get_param = |line_id: usize| lines[line_id].rsplit_once(' ').unwrap().1.parse().unwrap();
    for i in 0..14 {
        params[i] = Some(StepParams {
            do_cmp: get_param(i * 18 + 4) == 26,
            cmp_now: get_param(i * 18 + 5),
            cmp_later: get_param(i * 18 + 15),
        })
    }
    params.map(Option::unwrap)
}

#[derive(Debug, new, Clone, Copy)]
struct Rule {
    cmp_to: usize,
    val: i8,
}

fn get_rules(params: [StepParams; 14]) -> [Rule; 14] {
    let mut cmp_stack = Vec::new();
    let mut rules = [None; 14];
    for (i, step_params) in params.iter().enumerate() {
        if step_params.do_cmp {
            let Rule { cmp_to, val } = cmp_stack.pop().unwrap();
            rules[i] = Some(Rule::new(cmp_to, val + step_params.cmp_now));
            rules[cmp_to] = Some(Rule::new(i, -val - step_params.cmp_now));
        } else {
            cmp_stack.push(Rule::new(i, step_params.cmp_later))
        }
    }
    rules.map(Option::unwrap)
}

fn part_a(data: &'static str) -> String {
    let params = parse(data);
    let rules = get_rules(params);
    rules.map(|r| 9.min(9 + r.val).to_string()).concat()
}

fn part_b(data: &'static str) -> String {
    let params = parse(data);
    let rules = get_rules(params);
    rules.map(|r| 1.max(1 + r.val).to_string()).concat()
}
