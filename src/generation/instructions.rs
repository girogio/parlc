use std::fmt::Display;

pub enum Instructions {
    PushValue(u32),
    PushFunction(String),
    PushOffset(i32),
    PushFromStack(u32, u32),
    Store,
    NoOperation,
    Drop,
    Dup,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Inc,
    Dec,
    Max,
    Min,
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
    Halt,
    NewFrame,
    PopFrame,
    Alloc,
    Delay,
    Write,
    WriteBox,
    Clear,
    Width,
    Height,
    Print,
}

impl Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instructions::PushValue(value) => writeln!(f, "push {}", value),
            Instructions::PushFunction(name) => writeln!(f, "push {}", name),
            Instructions::PushOffset(offset) => {
                write!(f, "push ")?;
                match Ord::cmp(offset, &0) {
                    std::cmp::Ordering::Less => writeln!(f, "#PC-{}", offset.abs()),
                    std::cmp::Ordering::Equal => writeln!(f, "#PC"),
                    std::cmp::Ordering::Greater => writeln!(f, "#PC+{}", offset),
                }
            }
            Instructions::PushFromStack(frame_index, offset) => {
                writeln!(f, "push [{}:{}]", frame_index, offset)
            }
            Instructions::Store => writeln!(f, "st"),
            Instructions::NoOperation => writeln!(f, "nop"),
            Instructions::Drop => writeln!(f, "drop"),
            Instructions::Dup => writeln!(f, "dup"),
            Instructions::Add => writeln!(f, "add"),
            Instructions::Sub => writeln!(f, "sub"),
            Instructions::Mul => writeln!(f, "mul"),
            Instructions::Div => writeln!(f, "div"),
            Instructions::Mod => writeln!(f, "mod"),
            Instructions::Inc => writeln!(f, "inc"),
            Instructions::Dec => writeln!(f, "dec"),
            Instructions::Max => writeln!(f, "max"),
            Instructions::Min => writeln!(f, "min"),
            Instructions::RandInt => writeln!(f, "irnd"),
            Instructions::And => writeln!(f, "and"),
            Instructions::Or => writeln!(f, "or"),
            Instructions::Not => writeln!(f, "not"),
            Instructions::LessThan => writeln!(f, "lt"),
            Instructions::GreaterThan => writeln!(f, "gt"),
            Instructions::LessThanOrEqual => writeln!(f, "le"),
            Instructions::GreaterThanOrEqual => writeln!(f, "ge"),
            Instructions::Equal => writeln!(f, "eq"),
            Instructions::Jump => writeln!(f, "jmp"),
            Instructions::JumpIfNotZero => writeln!(f, "cjmp"),
            Instructions::Call => writeln!(f, "call"),
            Instructions::Return => writeln!(f, "ret"),
            Instructions::Halt => writeln!(f, "halt"),
            Instructions::NewFrame => writeln!(f, "oframe"),
            Instructions::PopFrame => writeln!(f, "cframe"),
            Instructions::Alloc => writeln!(f, "alloc"),
            Instructions::Delay => writeln!(f, "delay"),
            Instructions::Write => writeln!(f, "write"),
            Instructions::WriteBox => writeln!(f, "writebox"),
            Instructions::Clear => writeln!(f, "clear"),
            Instructions::Width => writeln!(f, "width"),
            Instructions::Height => writeln!(f, "height"),
            Instructions::Print => writeln!(f, "print"),
        }
    }
}
