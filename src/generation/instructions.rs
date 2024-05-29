use std::fmt::Display;

use crate::{core::Token, semantics::utils::MemoryLocation};

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Instruction>,
    pub main: Vec<Instruction>,
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for instruction in &self.functions {
            write!(f, "{}", instruction)?;
        }
        for instruction in &self.main {
            write!(f, "{}", instruction)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    FunctionLabel(String),
    PushArray(MemoryLocation),
    PushFromStack(MemoryLocation),
    PushFunction(Token),
    PushOffsetFromPC(i32),
    PushOffsetFromOpS(MemoryLocation),
    PushIntValue(usize),
    PushFloatValue(f64),
    Store,
    StoreArray,
    NoOperation,
    // Drop,
    // Dup,
    // Dupa,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    // Inc,
    // Dec,
    // Max,
    // Min,
    RandInt,            // Pop a, b; push random integer between a and b
    And,                // Pop a, b; push a == 1 && b == 1 ? 1 : 0
    Or,                 // Pop a, b; push a == 1 || b == 1 ? 1 : 0
    Not,                // Pop a; push a == 1 ? 0 : 1
    LessThan,           // Pop a, b; push a < b ? 1 : 0
    GreaterThan,        // Pop a, b; push a > b ? 1 : 0
    LessThanOrEqual,    // Pop a, b; push a <= b ? 1 : 0
    GreaterThanOrEqual, // Pop a, b; push a >= b ? 1 : 0
    Equal,              // Pop a, b; push a == b ? 1 : 0
    Jump,               // Pop a; jump to a
    JumpIfNotZero,      // Pop a, b; if b != 0, jump to a
    Call,
    Return,
    // ReturnArray,
    Halt,
    NewFrame,
    PopFrame,
    Alloc,
    Delay,
    Write,
    WriteBox,
    Clear,
    Width,
    Read,
    Height,
    Print,
    PrintArray,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Read => writeln!(f, "read"),
            Instruction::FunctionLabel(name) => writeln!(f, ".{}", name),
            Instruction::PushIntValue(value) => writeln!(f, "push {}", value),
            Instruction::PushFloatValue(value) => writeln!(f, "push {}", value),
            Instruction::PushArray(mem_loc) => {
                writeln!(f, "pusha [{}:{}]", mem_loc.frame_index, mem_loc.stack_level)
            }
            Instruction::PushFunction(name) => writeln!(f, "push .{}", name),
            Instruction::PushOffsetFromPC(offset) => {
                write!(f, "push ")?;
                match Ord::cmp(offset, &0) {
                    std::cmp::Ordering::Less => writeln!(f, "#PC-{}", offset.abs()),
                    std::cmp::Ordering::Equal => writeln!(f, "#PC"),
                    std::cmp::Ordering::Greater => writeln!(f, "#PC+{}", offset),
                }
            }
            Instruction::PushFromStack(mem_loc) => {
                writeln!(f, "push [{}:{}]", mem_loc.frame_index, mem_loc.stack_level)
            }
            Instruction::PushOffsetFromOpS(mem_loc) => {
                writeln!(f, "push +[{}:{}]", mem_loc.frame_index, mem_loc.stack_level)
            }
            Instruction::Store => writeln!(f, "st"),
            Instruction::StoreArray => writeln!(f, "sta"),
            Instruction::NoOperation => writeln!(f, "nop"),
            // Instruction::Drop => writeln!(f, "drop"),
            // Instruction::Dup => writeln!(f, "dup"),
            // Instruction::Dupa => writeln!(f, "dupa"),
            Instruction::Add => writeln!(f, "add"),
            Instruction::Sub => writeln!(f, "sub"),
            Instruction::Mul => writeln!(f, "mul"),
            Instruction::Div => writeln!(f, "div"),
            Instruction::Mod => writeln!(f, "mod"),
            // Instruction::Inc => writeln!(f, "inc"),
            // Instruction::Dec => writeln!(f, "dec"),
            // Instruction::Max => writeln!(f, "max"),
            // Instruction::Min => writeln!(f, "min"),
            Instruction::RandInt => writeln!(f, "irnd"),
            Instruction::And => writeln!(f, "and"),
            Instruction::Or => writeln!(f, "or"),
            Instruction::Not => writeln!(f, "not"),
            Instruction::LessThan => writeln!(f, "lt"),
            Instruction::GreaterThan => writeln!(f, "gt"),
            Instruction::LessThanOrEqual => writeln!(f, "le"),
            Instruction::GreaterThanOrEqual => writeln!(f, "ge"),
            Instruction::Equal => writeln!(f, "eq"),
            Instruction::Jump => writeln!(f, "jmp"),
            Instruction::JumpIfNotZero => writeln!(f, "cjmp"),
            Instruction::Call => writeln!(f, "call"),
            Instruction::Return => writeln!(f, "ret"),
            // Instruction::ReturnArray => writeln!(f, "reta"),
            Instruction::Halt => writeln!(f, "halt"),
            Instruction::NewFrame => writeln!(f, "oframe"),
            Instruction::PopFrame => writeln!(f, "cframe"),
            Instruction::Alloc => writeln!(f, "alloc"),
            Instruction::Delay => writeln!(f, "delay"),
            Instruction::Write => writeln!(f, "write"),
            Instruction::WriteBox => writeln!(f, "writebox"),
            Instruction::Clear => writeln!(f, "clear"),
            Instruction::Width => writeln!(f, "width"),
            Instruction::Height => writeln!(f, "height"),
            Instruction::Print => writeln!(f, "print"),
            Instruction::PrintArray => writeln!(f, "printa"),
        }
    }
}
