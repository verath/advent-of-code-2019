use std::mem;

const PARAM_MODE_POSITION: u8 = 0;
const PARAM_MODE_IMMEDIATE: u8 = 1;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Parameter {
    Position(usize),
    Immediate(i64),
}

fn position_param(position: i64) -> Parameter {
    assert!(position >= 0);
    Parameter::Position(position as usize)
}

fn immediate_param(value: i64) -> Parameter {
    Parameter::Immediate(value)
}

const OP_CODE_ADD: i64 = 1;
const OP_CODE_MUL: i64 = 2;
const OP_CODE_READ_INPUT: i64 = 3;
const OP_CODE_WRITE_OUTPUT: i64 = 4;
const OP_CODE_END: i64 = 99;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Operation {
    // Add param1 + param2, write to param3
    Add(Parameter, Parameter, Parameter),
    // Multiply param1 * param2, write to param3
    Mul(Parameter, Parameter, Parameter),
    // Read "input", write to param1
    ReadInput(Parameter),
    // Write "output" from param1.
    WriteOutput(Parameter),
    // End of program.
    End,
}

fn operation_size(op: Operation) -> usize {
    match op {
        Operation::Add(_, _, _) => 4,
        Operation::Mul(_, _, _) => 4,
        Operation::ReadInput(_) => 2,
        Operation::WriteOutput(_) => 2,
        Operation::End => 1,
    }
}

struct IntcodeContext {
    program: Vec<i64>,
    pc: usize,
    input: Vec<i64>,
    output: Vec<i64>,
}

impl IntcodeContext {
    pub fn new(program: &[i64], input: &[i64]) -> IntcodeContext {
        IntcodeContext {
            program: program.to_vec(),
            pc: 0,
            input: input.to_vec(),
            output: Vec::new(),
        }
    }

    fn parse_op_code(op_code_raw: i64) -> (i64, u8) {
        let mut addr_mode = op_code_raw / 100;
        // addr_mode is a "bitmask" in base 10, specifying addressing mode
        // for parameters incrementing right to left.
        let mut mask = 0;
        let mut i = 0;
        while addr_mode > 0 {
            let pos_set = ((addr_mode % 10) > 0) as u8;
            addr_mode /= 10;
            mask |= pos_set << i;
            i += 1;
        }
        let op_code = op_code_raw % 100;
        (op_code, mask)
    }

    fn make_param(val: i64, param_mode: u8) -> Parameter {
        match param_mode {
            PARAM_MODE_POSITION => position_param(val),
            PARAM_MODE_IMMEDIATE => immediate_param(val),
            _ => panic!("Unknown param_mode: {}", param_mode),
        }
    }

    fn parse_next(&mut self) -> Operation {
        let (op_code, addr_mode_mask) = Self::parse_op_code(self.program[self.pc]);
        let op = match op_code {
            OP_CODE_ADD => {
                let param1_mode = addr_mode_mask & 0b01;
                let param2_mode = (addr_mode_mask & 0b10) >> 1;
                let param1 = Self::make_param(self.program[self.pc + 1], param1_mode);
                let param2 = Self::make_param(self.program[self.pc + 2], param2_mode);
                let param3 = Self::make_param(self.program[self.pc + 3], PARAM_MODE_POSITION);
                Operation::Add(param1, param2, param3)
            }
            OP_CODE_MUL => {
                let param1_mode = addr_mode_mask & 0b01;
                let param2_mode = (addr_mode_mask & 0b10) >> 1;
                let param1 = Self::make_param(self.program[self.pc + 1], param1_mode);
                let param2 = Self::make_param(self.program[self.pc + 2], param2_mode);
                let param3 = Self::make_param(self.program[self.pc + 3], PARAM_MODE_POSITION);
                Operation::Mul(param1, param2, param3)
            }
            OP_CODE_READ_INPUT => {
                let param = Self::make_param(self.program[self.pc + 1], PARAM_MODE_POSITION);
                Operation::ReadInput(param)
            }
            OP_CODE_WRITE_OUTPUT => {
                let param_mode = addr_mode_mask & 0b01;
                let param = Self::make_param(self.program[self.pc + 1], param_mode);
                Operation::WriteOutput(param)
            }
            OP_CODE_END => Operation::End,
            _ => panic!("Invalid op_code {}", op_code),
        };
        self.pc += operation_size(op);
        op
    }

    fn resolve_param(&self, param: Parameter) -> i64 {
        match param {
            Parameter::Immediate(value) => value,
            Parameter::Position(pos) => self.program[pos],
        }
    }

    fn evaluate(&mut self, op: Operation) {
        match op {
            Operation::Add(p1, p2, Parameter::Position(out_pos)) => {
                let val1 = self.resolve_param(p1);
                let val2 = self.resolve_param(p2);
                self.program[out_pos] = val1 + val2;
            }
            Operation::Mul(p1, p2, Parameter::Position(out_pos)) => {
                let val1 = self.resolve_param(p1);
                let val2 = self.resolve_param(p2);
                self.program[out_pos] = val1 * val2;
            }
            Operation::ReadInput(Parameter::Position(to_pos)) => {
                self.program[to_pos] = self.input.pop().expect("no input");
            }
            Operation::WriteOutput(p) => {
                let val = self.resolve_param(p);
                self.output.push(val);
            }
            _ => panic!("Invalid operation: {:?}", op),
        }
    }

    fn run(&mut self) {
        self.pc = 0;
        loop {
            let op = self.parse_next();
            if op == Operation::End {
                break;
            }
            self.evaluate(op);
        }
    }
}

pub fn run(program: &[i64], input: &[i64]) -> (Vec<i64>, Vec<i64>) {
    let mut ctx = IntcodeContext::new(program, input);
    ctx.run();
    (
        mem::replace(&mut ctx.program, Vec::new()),
        mem::replace(&mut ctx.output, Vec::new()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_op_code() {
        assert_eq!(IntcodeContext::parse_op_code(1), (1, 0));
        assert_eq!(IntcodeContext::parse_op_code(1101), (1, 0b11));
        assert_eq!(IntcodeContext::parse_op_code(1001), (1, 0b10));
        assert_eq!(IntcodeContext::parse_op_code(101), (1, 0b1));
        assert_eq!(IntcodeContext::parse_op_code(11), (11, 0));
    }

    #[test]
    fn test_parse_next() {
        let mut ctx = IntcodeContext::new(
            &[
                1, 1, 2, 3, // ADD P(1) + P(2) => P(3)
                2, 1, 2, 3, // MUL P(1) + P(2) => P(3)
                3, 1, // ReadInput => P(1)
                4, 1, // WriteOutput <= P(1)
                1101, 1, 2, 3, // ADD IM(1) + IM(2) => P(3)
                1001, 1, 2, 3,  // ADD P(1) + IM(2) => P(3)
                99, // END
            ],
            &[],
        );
        assert_eq!(
            ctx.parse_next(),
            Operation::Add(
                Parameter::Position(1),
                Parameter::Position(2),
                Parameter::Position(3)
            )
        );
        assert_eq!(
            ctx.parse_next(),
            Operation::Mul(
                Parameter::Position(1),
                Parameter::Position(2),
                Parameter::Position(3)
            )
        );
        assert_eq!(
            ctx.parse_next(),
            Operation::ReadInput(Parameter::Position(1))
        );
        assert_eq!(
            ctx.parse_next(),
            Operation::WriteOutput(Parameter::Position(1))
        );
        assert_eq!(
            ctx.parse_next(),
            Operation::Add(
                Parameter::Immediate(1),
                Parameter::Immediate(2),
                Parameter::Position(3)
            )
        );
        assert_eq!(
            ctx.parse_next(),
            Operation::Add(
                Parameter::Position(1),
                Parameter::Immediate(2),
                Parameter::Position(3)
            )
        );
        assert_eq!(ctx.parse_next(), Operation::End);
    }

    #[test]
    fn test_run_day2() {
        let program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let expected = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        let (program, _) = run(&program, &[]);
        assert_eq!(program, expected);
    }

    #[test]
    fn test_run_input_output() {
        // Read input, write output
        let program = vec![3, 0, 4, 0, 99];
        let input = vec![1337];
        let expected_output = input.clone();
        let (_, output) = run(&program, &input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_run_negative_value() {
        // 100 + -1, store at pos 4.
        let program = vec![1101, 100, -1, 4, 0];
        let expected_program = vec![1101, 100, -1, 4, 99];
        let (program, _) = run(&program, &[]);
        assert_eq!(program, expected_program);
    }
}
