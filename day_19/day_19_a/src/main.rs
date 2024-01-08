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

    fn redirect(&self, part: &Part) -> String {
        for cond in self.conditions.iter() {
            if cond.validate(part) {
                return cond.result.clone();
            }
        }
        self.fin.clone()
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

    fn validate(&self, x: &Part) -> bool {
        let mut it = self.condition.chars().into_iter();
        let val = match it.next().unwrap() {
            'x' => x.x,
            'm' => x.m,
            'a' => x.a,
            's' => x.s,
            _ => unreachable!(),
        };
        let sign = it.next().unwrap();
        let num = it.collect::<String>().parse::<i32>().unwrap();
        match sign {
            '>' => return val > num,
            '<' => return val < num,
            _ => unreachable!(),
        };
    }
}

struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}
impl Part {
    fn build(line: &str) -> Self {
        let content = &line[1..line.len() - 1];
        let values = content.split(",").collect::<Vec<&str>>();
        Self {
            x: values[0][2..].parse::<i32>().unwrap(),
            m: values[1][2..].parse::<i32>().unwrap(),
            a: values[2][2..].parse::<i32>().unwrap(),
            s: values[3][2..].parse::<i32>().unwrap(),
        }
    }

    fn get_value(&self) -> i32 {
        self.x + self.m + self.a + self.s
    }
}
fn main() -> Result<(), std::io::Error> {
    let mut input = File::open("input.txt")?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;

    let mut parts = Vec::new();
    let mut workflows = HashMap::new();
    let mut is_workflow = true;
    for line in contents.lines() {
        if line.is_empty() {
            is_workflow = false;
            continue;
        }
        if is_workflow {
            let x = Workflow::build(line);
            workflows.insert(x.name.clone(), x);
        } else {
            parts.push(Part::build(line));
        }
    }

    let mut sum = 0;
    for part in parts.iter() {
        let mut wf = workflows.get("in".into()).unwrap();
        let mut res = wf.redirect(part);
        while res != "A".to_string() && res != "R".to_string() {
            wf = workflows.get(&res).unwrap();
            res = wf.redirect(part);
        }
        match res.chars().next().unwrap() {
            'A' => sum += part.get_value(),
            _ => (),
        }
    }
    println!("{sum}");

    Ok(())
}
