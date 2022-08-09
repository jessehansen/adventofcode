#![feature(drain_filter)]

use anyhow::*;
use aoc_common::*;
use std::collections::HashMap;
use std::str::FromStr;

fn main() -> Result<()> {
    run_progressive(parse_all, part1, part2)
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Expression {
    Literal(u16),
    Wire(String),
}

impl Expression {
    fn value(&self, wires: &HashMap<String, u16>) -> u16 {
        match self {
            Expression::Literal(value) => *value,
            Expression::Wire(name) => *wires.get(name).expect("cannot evaluate unsignaled wire"),
        }
    }
}

impl FromStr for Expression {
    type Err = Error;

    fn from_str(expression: &str) -> Result<Self> {
        if let std::result::Result::Ok(value) = expression.parse() {
            Ok(Expression::Literal(value))
        } else {
            Ok(Expression::Wire(expression.to_string()))
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Operation {
    Provide(Expression),
    And { left: Expression, right: Expression },
    Or { left: Expression, right: Expression },
    LShift { left: Expression, right: Expression },
    RShift { left: Expression, right: Expression },
    Not(Expression),
}

use Operation::*;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Gate {
    op: Operation,
    output: String,
}

impl Gate {
    fn has_signal(&self, wires: &HashMap<String, u16>) -> bool {
        let mut inputs = vec![];
        match &self.op {
            Provide(ex) => inputs.push(ex),
            And { left, right } => {
                inputs.push(left);
                inputs.push(right);
            }
            Or { left, right } => {
                inputs.push(left);
                inputs.push(right);
            }
            LShift { left, right } => {
                inputs.push(left);
                inputs.push(right);
            }
            RShift { left, right } => {
                inputs.push(left);
                inputs.push(right);
            }
            Not(ex) => inputs.push(ex),
        };
        inputs.iter().all(|x| match x {
            Expression::Literal(_) => true,
            Expression::Wire(name) => wires.contains_key(name),
        })
    }

    fn exec(&self, wires: &mut HashMap<String, u16>) {
        let out = match &self.op {
            Provide(expression) => expression.value(wires),
            And { left, right } => left.value(wires) & right.value(wires),
            Or { left, right } => left.value(wires) | right.value(wires),
            LShift { left, right } => left.value(wires) << right.value(wires),
            RShift { left, right } => left.value(wires) >> right.value(wires),
            Not(expression) => !expression.value(wires),
        };
        let wire = wires.entry(self.output.clone()).or_insert(0);
        *wire = out;
    }
}

impl FromStr for Gate {
    type Err = Error;

    fn from_str(gate: &str) -> Result<Self> {
        let mut in_out = gate.split(" -> ");

        let op = in_out.next().unwrap();
        let output = in_out.next().unwrap().to_string();

        let parts: Vec<&str> = op.split(' ').collect();
        let op = match parts.len() {
            1 => Provide(parts[0].parse().unwrap()),
            2 if parts[0] == "NOT" => Not(parts[1].parse().unwrap()),
            3 if parts[1] == "AND" => And {
                left: parts[0].parse().unwrap(),
                right: parts[2].parse().unwrap(),
            },
            3 if parts[1] == "OR" => Or {
                left: parts[0].parse().unwrap(),
                right: parts[2].parse().unwrap(),
            },
            3 if parts[1] == "LSHIFT" => LShift {
                left: parts[0].parse().unwrap(),
                right: parts[2].parse().unwrap(),
            },
            3 if parts[1] == "RSHIFT" => RShift {
                left: parts[0].parse().unwrap(),
                right: parts[2].parse().unwrap(),
            },
            op => bail!("unsupported operation '{}'", op),
        };

        Ok(Gate { op, output })
    }
}

#[derive(Clone, Debug)]
struct Circuit {
    gates: Vec<Gate>,
}

impl Circuit {
    fn override_output(&mut self, name: &str, value: u16) {
        let name = name.to_string();
        self.gates.drain_filter(|gate| gate.output == name);
        self.gates.insert(
            0,
            Gate {
                op: Provide(Expression::Literal(value)),
                output: name,
            },
        );
    }

    fn run(&self) -> HashMap<String, u16> {
        let mut wires = HashMap::new();
        let mut gates = self.gates.clone();
        while !gates.is_empty() {
            gates.drain_filter(|gate| {
                if gate.has_signal(&wires) {
                    gate.exec(&mut wires);
                    true
                } else {
                    false
                }
            });
        }
        wires
    }
}

impl FromStr for Circuit {
    type Err = Error;

    fn from_str(circuit: &str) -> Result<Self> {
        let gates = parse_lines(circuit)?;

        Ok(Circuit { gates })
    }
}

fn part1(circuit: &Circuit) -> Result<(u16, u16)> {
    let wires = circuit.run();

    let a = wires["a"];

    Ok((a, a))
}

fn part2(circuit: &Circuit, new_b: &u16) -> Result<u16> {
    let mut circuit = circuit.clone();
    circuit.override_output("b", *new_b);
    let wires = circuit.run();

    Ok(wires["a"])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circuit_run() -> Result<()> {
        let circuit: Circuit = SAMPLE.parse()?;

        let wires = circuit.run();

        assert_eq!(wires["d"], 72);
        assert_eq!(wires["e"], 507);
        assert_eq!(wires["f"], 492);
        assert_eq!(wires["g"], 114);
        assert_eq!(wires["h"], 65412);
        assert_eq!(wires["i"], 65079);
        assert_eq!(wires["x"], 123);
        assert_eq!(wires["y"], 456);

        Ok(())
    }

    const SAMPLE: &str = "\
123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
";
}
