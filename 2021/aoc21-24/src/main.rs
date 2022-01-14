use aoc_common::*;
use std::collections::HashSet;
use std::str::FromStr;

fn main() {
    run(prepare, part1, part2);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Operand {
    Register(char),
    Literal(i64),
}

impl FromStr for Operand {
    type Err = ();

    fn from_str(input: &str) -> Result<Operand, Self::Err> {
        match input {
            "w" => Ok(Operand::Register('w')),
            "x" => Ok(Operand::Register('x')),
            "y" => Ok(Operand::Register('y')),
            "z" => Ok(Operand::Register('z')),
            num => Ok(Operand::Literal(num.parse().unwrap())),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Instruction {
    Inp(Operand),
    Add(Operand, Operand),
    Mul(Operand, Operand),
    Div(Operand, Operand),
    Mod(Operand, Operand),
    Eql(Operand, Operand),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(input: &str) -> Result<Instruction, Self::Err> {
        let mut parts = input.split(' ');
        match parts.next().unwrap() {
            "inp" => Ok(Instruction::Inp(parts.next().unwrap().parse().unwrap())),
            "add" => Ok(Instruction::Add(
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )),
            "mul" => Ok(Instruction::Mul(
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )),
            "div" => Ok(Instruction::Div(
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )),
            "mod" => Ok(Instruction::Mod(
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )),
            "eql" => Ok(Instruction::Eql(
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )),
            x => panic!("unknown instruction '{}'", x),
        }
    }
}

struct ArithmeticLogicUnit {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl ArithmeticLogicUnit {
    fn new() -> ArithmeticLogicUnit {
        ArithmeticLogicUnit {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    fn run<T>(&mut self, program: &[Instruction], input: T)
    where
        T: IntoIterator<Item = i64>,
    {
        let mut input = input.into_iter();
        for instruction in program {
            match instruction {
                Instruction::Inp(Operand::Register(c)) => {
                    *self.reg_mut(*c) = input.next().unwrap();
                }
                Instruction::Add(Operand::Register(c), rhs) => {
                    let val = self.val(rhs);
                    let lhs = self.reg_mut(*c);
                    *lhs += val;
                }
                Instruction::Mul(Operand::Register(c), rhs) => {
                    let val = self.val(rhs);
                    let lhs = self.reg_mut(*c);
                    *lhs *= val;
                }
                Instruction::Div(Operand::Register(c), rhs) => {
                    let val = self.val(rhs);
                    let lhs = self.reg_mut(*c);
                    *lhs /= val;
                }
                Instruction::Mod(Operand::Register(c), rhs) => {
                    let val = self.val(rhs);
                    let lhs = self.reg_mut(*c);
                    *lhs %= val;
                }
                Instruction::Eql(Operand::Register(c), rhs) => {
                    let val = self.val(rhs);
                    let lhs = self.reg_mut(*c);
                    *lhs = if *lhs == val { 1 } else { 0 }
                }
                _ => panic!(),
            }
        }
    }

    fn reg_mut(&mut self, c: char) -> &mut i64 {
        match c {
            'w' => &mut self.w,
            'x' => &mut self.x,
            'y' => &mut self.y,
            'z' => &mut self.z,
            _ => panic!(),
        }
    }

    fn val(&self, value: &Operand) -> i64 {
        match value {
            Operand::Register('w') => self.w,
            Operand::Register('x') => self.x,
            Operand::Register('y') => self.y,
            Operand::Register('z') => self.z,
            Operand::Literal(v) => *v,
            _ => panic!(),
        }
    }

    fn reset(&mut self) {
        self.w = 0;
        self.x = 0;
        self.y = 0;
        self.z = 0;
    }
}

fn parse(contents: &str) -> Vec<Instruction> {
    contents.lines().map(|x| x.parse().unwrap()).collect()
}

fn prepare(contents: &str) -> (Vec<Instruction>, Vec<HashSet<i64>>) {
    let program = parse(contents);

    // model no. program is split into chunks of 18 instructions each. Each chunk looks at the z
    // output from the previous chunk and runs a bunch of instructions to set z for the next chunk
    //
    // with this knowledge, we can work backwards to find valid z inputs for each chunk
    let mut alu = ArithmeticLogicUnit::new();
    let mut chunk_index = 13;
    let mut valid_z: Vec<HashSet<i64>> = vec![HashSet::new(); 14];
    valid_z[13].insert(0);
    while chunk_index > 0 {
        let chunk = &program[chunk_index * 18..(chunk_index + 1) * 18];
        for digit in 1..=9 {
            for input_z in 0..1000000 {
                alu.z = input_z;
                alu.run(chunk, [digit]);
                if valid_z[chunk_index].contains(&alu.z) {
                    valid_z[chunk_index - 1].insert(input_z);
                }

                alu.reset();
            }
        }

        chunk_index -= 1;
    }

    (program, valid_z)
}

fn find_model_number<F, T>(
    program: &[Instruction],
    valid_z: &[HashSet<i64>],
    digit_search: F,
) -> String
where
    F: Fn() -> T,
    T: Iterator<Item = i64>,
{
    let mut alu = ArithmeticLogicUnit::new();

    let mut chunk_index = 0;
    let mut final_answer = vec![];
    let mut prev_z = 0;

    while chunk_index < 14 {
        let mut found = false;
        for digit in digit_search() {
            alu.z = prev_z;
            let chunk = &program[chunk_index * 18..(chunk_index + 1) * 18];
            alu.run(chunk, [digit]);

            if valid_z[chunk_index].contains(&alu.z) {
                prev_z = alu.z;
                final_answer.push(digit);
                alu.reset();
                found = true;
                break;
            }
            alu.reset();
        }
        if !found {
            println!("no digit found for chunk {}", chunk_index);
        }
        chunk_index += 1;
    }

    final_answer
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn part1((program, valid_z): &(Vec<Instruction>, Vec<HashSet<i64>>)) -> String {
    find_model_number(program, valid_z, || (1..=9).rev())
}

fn part2((program, valid_z): &(Vec<Instruction>, Vec<HashSet<i64>>)) -> String {
    find_model_number(program, valid_z, || 1..=9)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alu_neg() {
        let program = parse(NEG);

        let mut alu = ArithmeticLogicUnit::new();
        alu.run(&program, [1]);

        assert_eq!(alu.x, -1);
    }

    #[test]
    fn alu_mul_eql() {
        let program = parse(MUL_EQL);

        let mut alu = ArithmeticLogicUnit::new();
        alu.run(&program, [1, 3]);

        assert_eq!(alu.x, 3);
        assert_eq!(alu.z, 1);

        alu.run(&program, [1, 2]);

        assert_eq!(alu.x, 2);
        assert_eq!(alu.z, 0);
    }

    #[test]
    fn alu_bit_split() {
        let program = parse(BIT_SPLIT);

        let mut alu = ArithmeticLogicUnit::new();
        alu.run(&program, [0b1011]);

        assert_eq!(alu.w, 1);
        assert_eq!(alu.x, 0);
        assert_eq!(alu.y, 1);
        assert_eq!(alu.z, 1);
    }

    const NEG: &str = "\
inp x
mul x -1
";

    const MUL_EQL: &str = "\
inp z
inp x
mul z 3
eql z x
";

    const BIT_SPLIT: &str = "\
inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2
";
}
