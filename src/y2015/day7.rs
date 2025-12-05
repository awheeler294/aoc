use std::collections::HashMap;

pub fn solve(input: &[&str]) -> String {
    let (part1, part2) = run_circuit(input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

#[derive(Clone, PartialEq, Debug)]
enum Op {
    Assignment(String),
    AND(String, String),
    OR(String, String),
    NOT(String),
    LSHIFT(String, u16),
    RSHIFT(String, u16),
}

fn run_circuit(instructions: &[&str]) -> (u16, u16) {
    let mut circuit = parse_instructions(instructions).unwrap();

    let first_val = evaluate(&circuit, "a");

    circuit.insert("b", Op::Assignment(first_val.to_string()));
    let second_val = evaluate(&circuit, "a");

    (first_val, second_val)
}

fn parse_instructions<'a>(instructions: &[&'a str]) -> Result<HashMap<&'a str, Op>, String> {
    let mut circuit = HashMap::new();

    for instruction in instructions {
        let token = instruction.split(' ').nth(0).ok_or(format!(
            "Could not parse {}tn token from {}",
            0, instruction
        ))?;

        // Assignment: 123 -> x, l -> k
        if instruction.split(' ').nth(1).ok_or(format!(
            "Could not parse {}tn token from {}",
            2, instruction
        ))? == "->"
        {
            let lhs = token;
            let rhs = instruction.split(' ').nth(2).ok_or(format!(
                "Could not parse {}tn token from {}",
                2, instruction
            ))?;
            circuit.insert(rhs, Op::Assignment(lhs.to_string()));
        }
        //if let Ok(value) = token.parse::<u16>() {
        //    let wire_name = instruction.split(' ').nth(2)
        //        .ok_or(format!("Could not parse {}tn token from {}", 2, instruction))?;
        //    circuit.insert(wire_name, Op::Value(value));
        //}

        // NOT: NOT x -> h
        else if token == "NOT" {
            let lhs = instruction.split(' ').nth(1).ok_or(format!(
                "Could not parse {}tn token from {}",
                1, instruction
            ))?;
            let rhs = instruction.split(' ').nth(3).ok_or(format!(
                "Could not parse {}tn token from {}",
                3, instruction
            ))?;
            circuit.insert(rhs, Op::NOT(lhs.to_string()));
        }
        // Other Operations
        else {
            let lhs = token;
            let operator = instruction.split(' ').nth(1).ok_or(format!(
                "Could not parse {}tn token from {}",
                1, instruction
            ))?;
            let rhs = instruction.split(' ').nth(2).ok_or(format!(
                "Could not parse {}tn token from {}",
                2, instruction
            ))?;
            let wire_name = instruction.split(' ').nth(4).ok_or(format!(
                "Could not parse {}tn token from {}",
                4, instruction
            ))?;

            let op = match operator {
                "AND" => Op::AND(lhs.to_string(), rhs.to_string()),
                "OR" => Op::OR(lhs.to_string(), rhs.to_string()),
                "LSHIFT" => Op::LSHIFT(lhs.to_string(), rhs.parse::<u16>().unwrap()),
                "RSHIFT" => Op::RSHIFT(lhs.to_string(), rhs.parse::<u16>().unwrap()),
                _ => return Err(format!("Unknown operator {}", operator)),
            };

            circuit.insert(wire_name, op);
        }
    }

    Ok(circuit)
}

fn evaluate(circuit: &HashMap<&str, Op>, wire: &str) -> u16 {
    evaluate_rec(circuit, &mut HashMap::new(), wire)
}

fn evaluate_rec(circuit: &HashMap<&str, Op>, values: &mut HashMap<String, u16>, wire: &str) -> u16 {
    if let Some(value) = values.get(wire) {
        return *value;
    }

    //dbg!(&wire);
    if let Ok(value) = wire.parse::<u16>() {
        //dbg!(&value);
        return value;
    }

    let wire_value = circuit.get(wire).unwrap();
    //dbg!(&wire_value);
    match wire_value {
        Op::Assignment(operand) => {
            let value = evaluate_rec(circuit, values, operand);
            values.insert(operand.to_string(), value);

            value
        }
        Op::AND(lhs, rhs) => {
            let lhv = evaluate_rec(circuit, values, lhs);
            values.insert(lhs.to_string(), lhv);

            let rhv = evaluate_rec(circuit, values, rhs);
            values.insert(rhs.to_string(), rhv);

            lhv & rhv
        }
        Op::OR(lhs, rhs) => {
            let lhv = evaluate_rec(circuit, values, lhs);
            values.insert(lhs.to_string(), lhv);

            let rhv = evaluate_rec(circuit, values, rhs);
            values.insert(rhs.to_string(), rhv);

            lhv | rhv
        }
        Op::NOT(operand) => {
            let value = evaluate_rec(circuit, values, operand);
            values.insert(operand.to_string(), value);

            !value
        }
        Op::LSHIFT(operand, shift_ammount) => {
            let value = evaluate_rec(circuit, values, operand);
            values.insert(operand.to_string(), value);

            value << shift_ammount
        }
        Op::RSHIFT(operand, shift_ammount) => {
            let value = evaluate_rec(circuit, values, operand);
            values.insert(operand.to_string(), value);

            value >> shift_ammount
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_circuit() {
        let instructions = vec!["123 -> x"];
        let expected_circuit = HashMap::from([("x", Op::Assignment("123".to_string()))]);
        let circuit = parse_instructions(&instructions).unwrap();
        assert_eq!(circuit, expected_circuit);

        let result = evaluate(&circuit, "x");
        assert_eq!(result, 123);

        let instructions = vec!["1 AND r -> s"];
        let expected_circuit = HashMap::from([("s", Op::AND("1".to_string(), "r".to_string()))]);
        let circuit = parse_instructions(&instructions).unwrap();
        assert_eq!(circuit, expected_circuit);

        #[rustfmt::skip]
        let instructions = vec![
            "123 -> x",
            "456 -> y",
            "x AND y -> d",
            "x OR y -> e",
            "x LSHIFT 2 -> f",
            "y RSHIFT 2 -> g",
            "NOT x -> h",
            "NOT y -> i",
        ];
        let circuit = parse_instructions(&instructions).unwrap();

        let result = evaluate(&circuit, "d");
        assert_eq!(result, 72);
        let result = evaluate(&circuit, "e");
        assert_eq!(result, 507);
        let result = evaluate(&circuit, "f");
        assert_eq!(result, 492);
        let result = evaluate(&circuit, "g");
        assert_eq!(result, 114);
        let result = evaluate(&circuit, "h");
        assert_eq!(result, 65412);
        let result = evaluate(&circuit, "i");
        assert_eq!(result, 65079);
        let result = evaluate(&circuit, "x");
        assert_eq!(result, 123);
        let result = evaluate(&circuit, "y");
        assert_eq!(result, 456);
    }
}
