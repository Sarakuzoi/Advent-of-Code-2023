use std::{collections::HashMap, fs::File, io::Read};
struct Workflow {
    name: String,
    conditions: Vec<Rule>,
    fin: String,
}

impl Workflow {
    fn build(line: &str) -> Self {
        let name = line[..line.find("{").unwrap()].to_string();
        let contents = line[line.find("{").unwrap() + 1..line.find("}").unwrap()].to_string();
        let mut conditions_raw = contents.split(",").collect::<Vec<&str>>();
        let fin = conditions_raw.last().unwrap().to_string();
        conditions_raw.pop();

        let mut conditions = Vec::new();
        for cond in conditions_raw {
            conditions.push(Rule::build(cond));
        }

        Self {
            name,
            conditions,
            fin,
        }
    }
}

struct Rule {
    condition: String,
    result: String,
}

impl Rule {
    fn build(s: &str) -> Self {
        let mut aux = s.split(":").into_iter();
        let condition = aux.next().unwrap().to_string();
        let result = aux.next().unwrap().to_string();

        Self { condition, result }
    }
}

#[derive(Clone)]
struct Part {
    x: (i64, i64),
    m: (i64, i64),
    a: (i64, i64),
    s: (i64, i64),
}

impl std::fmt::Debug for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Part")
            .field("x", &self.x)
            .field("m", &self.m)
            .field("a", &self.a)
            .field("s", &self.s)
            .finish()
    }
}
fn main() -> Result<(), std::io::Error> {
    let mut input = File::open("input.txt")?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;

    let mut workflows = HashMap::new();
    for line in contents.lines() {
        if line.is_empty() {
            break;
        }
        let x = Workflow::build(line);
        workflows.insert(x.name.clone(), x);
    }
    workflows.insert(
        "A".to_string(),
        Workflow {
            name: String::from("A"),
            conditions: Vec::new(),
            fin: String::new(),
        },
    );
    workflows.insert(
        "R".to_string(),
        Workflow {
            name: String::from("R"),
            conditions: Vec::new(),
            fin: String::new(),
        },
    );

    let mut sum = 0;
    let mut stack = vec![(
        workflows.get("in").unwrap(),
        Part {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        },
    )];

    while !stack.is_empty() {
        let (workflow, mut part) = stack.pop().unwrap();
        if &workflow.name == "A" {
            sum += (part.x.1 - part.x.0 + 1).max(0)
                * (part.m.1 - part.m.0 + 1).max(0)
                * (part.a.1 - part.a.0 + 1).max(0)
                * (part.s.1 - part.s.0 + 1).max(0);
            continue;
        }
        if &workflow.name == "R" {
            continue;
        }
        for i in 0..workflow.conditions.len() {
            let (cond, res) = (
                &workflow.conditions[i].condition,
                &workflow.conditions[i].result,
            );
            let mut it = cond.chars();
            let (attr, sign, num) = (
                it.next().unwrap(),
                it.next().unwrap(),
                cond[2..].parse::<i64>().unwrap(),
            );
            let next_wf = workflows.get(res).unwrap();
            match attr {
                'x' => {
                    let prev_part = part.clone();
                    part.x = match sign {
                        '>' => (num + 1, part.x.1),
                        '<' => (part.x.0, num - 1),
                        _ => unreachable!(),
                    };
                    stack.push((next_wf, part.clone()));
                    part.x = match sign {
                        '<' => (num, prev_part.x.1),
                        '>' => (prev_part.x.0, num),
                        _ => unreachable!(),
                    };
                }

                'm' => {
                    let prev_part = part.clone();
                    part.m = match sign {
                        '>' => (num + 1, part.m.1),
                        '<' => (part.m.0, num - 1),
                        _ => unreachable!(),
                    };
                    stack.push((next_wf, part.clone()));
                    part.m = match sign {
                        '<' => (num, prev_part.m.1),
                        '>' => (prev_part.m.0, num),
                        _ => unreachable!(),
                    };
                }

                'a' => {
                    let prev_part = part.clone();
                    part.a = match sign {
                        '>' => (num + 1, part.a.1),
                        '<' => (part.a.0, num - 1),
                        _ => unreachable!(),
                    };
                    stack.push((next_wf, part.clone()));
                    part.a = match sign {
                        '<' => (num, prev_part.a.1),
                        '>' => (prev_part.a.0, num),
                        _ => unreachable!(),
                    };
                }

                's' => {
                    let prev_part = part.clone();
                    part.s = match sign {
                        '>' => (num + 1, part.s.1),
                        '<' => (part.s.0, num - 1),
                        _ => unreachable!(),
                    };
                    stack.push((next_wf, part.clone()));
                    part.s = match sign {
                        '<' => (num, prev_part.s.1),
                        '>' => (prev_part.s.0, num),
                        _ => unreachable!(),
                    };
                }

                _ => unreachable!(),
            }
        }
        stack.push((workflows.get(&workflow.fin).unwrap(), part.clone()))
    }

    println!("{sum}");

    Ok(())
}
