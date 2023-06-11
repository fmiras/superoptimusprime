use regex::Regex;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

use crate::cpu::Instruction;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidOp,
    NoArgs,
    Regex(regex::Error),
    ArgParse(ParseIntError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid operation")
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\w+)\s+([-\d]+)(?:,\s*([-\d]+)(?:,\s*([-\d]+))?)?")
            .map_err(|e| ParseError::Regex(e))?;

        let caps = re.captures(s);
        if caps.is_none() {
            return Err(ParseError::NoArgs);
        }
        let caps = caps.unwrap();
        let op_str = &caps[1];

        let args: Vec<usize> = caps
            .iter()
            .skip(2)
            .filter_map(|m| m)
            .map(|m| {
                m.as_str()
                    .parse::<usize>()
                    .map_err(|e| ParseError::ArgParse(e))
            })
            .collect::<Result<Vec<usize>, ParseError>>()?;

        match op_str {
            "LOAD" => Ok(Instruction::Load(args[0] as i32)),
            "SWAP" => Ok(Instruction::Swap(args[0], args[1])),
            "XOR" => Ok(Instruction::Xor(args[0], args[1])),
            "INC" => Ok(Instruction::Inc(args[0])),
            _ => Err(ParseError::InvalidOp),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::Load(val) => write!(f, "LOAD {}", val),
            Instruction::Swap(mem1, mem2) => write!(f, "SWAP {}, {}", mem1, mem2),
            Instruction::Xor(mem1, mem2) => write!(f, "XOR {}, {}", mem1, mem2),
            Instruction::Inc(mem) => write!(f, "INC {}", mem),
        }
    }
}

pub fn parse(assembly: &str) -> Result<Vec<Instruction>, ParseError> {
    assembly
        .lines()
        .map(|line| line.parse::<Instruction>())
        .collect::<Result<Vec<Instruction>, ParseError>>()
}

pub fn output(program: &Vec<Instruction>) -> String {
    program
        .iter()
        .map(|op| op.to_string())
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_valid_operation() {
        let op = "LOAD 0".parse::<Instruction>();
        assert!(op.is_ok());
        assert_eq!(op.unwrap(), Instruction::Load(0));
    }

    #[test]
    fn from_str_invalid_operation() {
        let op = "INVALID 1".parse::<Instruction>();
        assert!(op.is_err());
        let error = op.unwrap_err();
        assert_eq!(error, ParseError::InvalidOp);
    }

    #[test]
    fn from_str_no_args_operation() {
        let op = "INC".parse::<Instruction>();
        assert!(op.is_err());
        let error = op.unwrap_err();
        assert_eq!(error, ParseError::NoArgs);
    }

    #[test]
    fn can_parse() {
        let assembly = "LOAD 0\nSWAP 1, 2\nXOR 3, 4\nINC 5";
        let result = parse(assembly);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        let expected = vec![
            Instruction::Load(0),
            Instruction::Swap(1, 2),
            Instruction::Xor(3, 4),
            Instruction::Inc(5),
        ];
        assert!(parsed.iter().all(|op| expected.contains(op)));
    }

    #[test]
    fn cant_parse_invalid() {
        let assembly = "LOAD 0\nSWAP 1, 2\nXOR 3, 4\nINVALID 1\nINC 5";
        let result = parse(assembly);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error, ParseError::InvalidOp);
    }

    #[test]
    fn can_output() {
        let program = vec![
            Instruction::Load(0),
            Instruction::Swap(1, 2),
            Instruction::Xor(3, 4),
            Instruction::Inc(5),
        ];
        let output = output(&program);
        let expected = "LOAD 0\nSWAP 1, 2\nXOR 3, 4\nINC 5";
        assert_eq!(output, expected);
    }

    #[test]
    fn can_parse_and_output() {
        let assembly = "LOAD 0\nSWAP 1, 2\nXOR 3, 4\nINC 5";
        let result = parse(assembly);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        let output = output(&parsed);
        let expected = "LOAD 0\nSWAP 1, 2\nXOR 3, 4\nINC 5";
        assert_eq!(output, expected);
    }
}
