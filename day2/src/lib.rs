#[derive(Debug, Copy, Clone, PartialEq)]
enum Operation {
    Add(usize, usize, usize),
    Mul(usize, usize, usize),
    End,
}

fn parse_operation(op_data: &[u64]) -> Operation {
    let op_code = op_data[0];
    match op_code {
        1 => Operation::Add(
            op_data[1] as usize,
            op_data[2] as usize,
            op_data[3] as usize,
        ),
        2 => Operation::Mul(
            op_data[1] as usize,
            op_data[2] as usize,
            op_data[3] as usize,
        ),
        99 => Operation::End,
        _ => panic!("Invalid op_code {}", op_code),
    }
}

fn evaluate_operation(program: &mut [u64], op: Operation) -> bool {
    match op {
        Operation::Add(a1, a2, s) => {
            program[s] = program[a1] + program[a2];
            true
        }
        Operation::Mul(f1, f2, p) => {
            program[p] = program[f1] * program[f2];
            true
        }
        Operation::End => false,
    }
}

pub fn run(mut program: &mut [u64]) {
    let mut pc = 0;
    loop {
        let op = parse_operation(&program[pc..]);
        if !evaluate_operation(&mut program, op) {
            break;
        }
        pc += 4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_operation() {
        assert_eq!(parse_operation(&[1, 1, 2, 3]), Operation::Add(1, 2, 3));
        assert_eq!(parse_operation(&[2, 1, 2, 3]), Operation::Mul(1, 2, 3));
        assert_eq!(parse_operation(&[99]), Operation::End);
    }

    #[test]
    fn test_evaluate_operation() {
        let mut program = vec![10, 10, 0, 0];
        let op_add = Operation::Add(0, 1, 2);
        let op_mul = Operation::Mul(0, 1, 3);
        let op_end = Operation::End;
        let expected = vec![10, 10, 20, 100];
        assert_eq!(evaluate_operation(&mut program, op_add), true);
        assert_eq!(evaluate_operation(&mut program, op_mul), true);
        assert_eq!(evaluate_operation(&mut program, op_end), false);
        assert_eq!(program, expected);
    }

    #[test]
    fn test_run() {
        let mut program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let expected = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        run(&mut program);
        assert_eq!(program, expected);
    }
}
