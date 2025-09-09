use super::cpu::SimpleCPU;
use std::collections::HashMap;

/// Simple expression types
#[derive(Debug, Clone)]
pub enum Expr {
    Number(u8),
    Variable(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Xor(Box<Expr>, Box<Expr>),
}

/// Simple statement types
#[derive(Debug, Clone)]
pub enum Statement {
    Let(String, Expr),           // let x = 5
    Assign(String, Expr),        // x = y + 3
    Print(Expr),                 // print(x)
}

/// Simple interpreter for high-level language
pub struct Interpreter {
    cpu: SimpleCPU,
    variables: HashMap<String, u8>,
    memory_counter: u8,  // Track next available memory address
}

impl Interpreter {
    /// Create a new interpreter
    pub fn new() -> Self {
        Interpreter {
            cpu: SimpleCPU::new(),
            variables: HashMap::new(),
            memory_counter: 0,
        }
    }

    /// Execute a list of statements
    pub fn execute(&mut self, statements: Vec<Statement>) -> Result<(), String> {
        // Reset CPU and variables
        self.cpu.reset();
        self.variables.clear();
        self.memory_counter = 0;

        for statement in statements {
            self.execute_statement(statement)?;
        }

        Ok(())
    }

    /// Execute a single statement
    fn execute_statement(&mut self, statement: Statement) -> Result<(), String> {
        match statement {
            Statement::Let(var_name, expr) => {
                let value = self.evaluate_expression(expr)?;
                self.variables.insert(var_name, value);
                Ok(())
            }
            Statement::Assign(var_name, expr) => {
                if !self.variables.contains_key(&var_name) {
                    return Err(format!("Variable '{}' not declared", var_name));
                }
                let value = self.evaluate_expression(expr)?;
                self.variables.insert(var_name, value);
                Ok(())
            }
            Statement::Print(expr) => {
                let value = self.evaluate_expression(expr)?;
                println!("Result: {}", value);
                Ok(())
            }
        }
    }

    /// Evaluate an expression using the neural CPU
    fn evaluate_expression(&mut self, expr: Expr) -> Result<u8, String> {
        match expr {
            Expr::Number(n) => {
                if n > 15 {
                    Err(format!("Number {} exceeds 4-bit range (0-15)", n))
                } else {
                    Ok(n)
                }
            }
            Expr::Variable(var_name) => {
                self.variables.get(&var_name)
                    .copied()
                    .ok_or_else(|| format!("Variable '{}' not found", var_name))
            }
            Expr::Add(left, right) => {
                let a = self.evaluate_expression(*left)?;
                let b = self.evaluate_expression(*right)?;
                self.neural_add(a, b)
            }
            Expr::Sub(left, right) => {
                let a = self.evaluate_expression(*left)?;
                let b = self.evaluate_expression(*right)?;
                self.neural_sub(a, b)
            }
            Expr::And(left, right) => {
                let a = self.evaluate_expression(*left)?;
                let b = self.evaluate_expression(*right)?;
                self.neural_and(a, b)
            }
            Expr::Or(left, right) => {
                let a = self.evaluate_expression(*left)?;
                let b = self.evaluate_expression(*right)?;
                self.neural_or(a, b)
            }
            Expr::Xor(left, right) => {
                let a = self.evaluate_expression(*left)?;
                let b = self.evaluate_expression(*right)?;
                self.neural_xor(a, b)
            }
        }
    }

    /// Perform addition using neural CPU
    fn neural_add(&mut self, a: u8, b: u8) -> Result<u8, String> {
        // Generate program: load a, store in temp1, load b, store in temp2, load temp1, add temp2
        let temp1 = self.get_temp_address();
        let temp2 = self.get_temp_address();
        let result_addr = self.get_temp_address();

        let program = vec![
            0xA, a,        // LDI a
            0x2, temp1,    // STORE temp1
            0xA, b,        // LDI b  
            0x2, temp2,    // STORE temp2
            0x1, temp1,    // LOAD temp1
            0x3, temp2,    // ADD temp2 (neural addition!)
            0x2, result_addr, // STORE result
            0xF,           // HALT
        ];

        self.cpu.reset();
        self.cpu.load_program(&program);
        self.cpu.run(20);

        let result = self.cpu.memory.read(result_addr);
        println!("    Neural ADD: {} + {} = {} (addresses: temp1=0x{:X}, temp2=0x{:X}, result=0x{:X})", 
                 a, b, result, temp1, temp2, result_addr);
        
        Ok(result)
    }

    /// Perform subtraction using neural CPU
    fn neural_sub(&mut self, a: u8, b: u8) -> Result<u8, String> {
        let temp1 = self.get_temp_address();
        let temp2 = self.get_temp_address();
        let result_addr = self.get_temp_address();

        let program = vec![
            0xA, a,        // LDI a
            0x2, temp1,    // STORE temp1
            0xA, b,        // LDI b
            0x2, temp2,    // STORE temp2
            0x1, temp1,    // LOAD temp1
            0x4, temp2,    // SUB temp2 (neural subtraction!)
            0x2, result_addr, // STORE result
            0xF,           // HALT
        ];

        self.cpu.reset();
        self.cpu.load_program(&program);
        self.cpu.run(20);

        Ok(self.cpu.memory.read(result_addr))
    }

    /// Perform AND using neural CPU
    fn neural_and(&mut self, a: u8, b: u8) -> Result<u8, String> {
        let temp1 = self.get_temp_address();
        let temp2 = self.get_temp_address();
        let result_addr = self.get_temp_address();

        let program = vec![
            0xA, a,        // LDI a
            0x2, temp1,    // STORE temp1
            0xA, b,        // LDI b
            0x2, temp2,    // STORE temp2
            0x1, temp1,    // LOAD temp1
            0x5, temp2,    // AND temp2 (neural AND!)
            0x2, result_addr, // STORE result
            0xF,           // HALT
        ];

        self.cpu.reset();
        self.cpu.load_program(&program);
        self.cpu.run(20);

        let result = self.cpu.memory.read(result_addr);
        println!("    Neural AND: {} & {} = {} (binary: {:04b} & {:04b} = {:04b})", 
                 a, b, result, a, b, result);
        
        Ok(result)
    }

    /// Perform OR using neural CPU
    fn neural_or(&mut self, a: u8, b: u8) -> Result<u8, String> {
        let temp1 = self.get_temp_address();
        let temp2 = self.get_temp_address();
        let result_addr = self.get_temp_address();

        let program = vec![
            0xA, a,        // LDI a
            0x2, temp1,    // STORE temp1
            0xA, b,        // LDI b
            0x2, temp2,    // STORE temp2
            0x1, temp1,    // LOAD temp1
            0x6, temp2,    // OR temp2 (neural OR!)
            0x2, result_addr, // STORE result
            0xF,           // HALT
        ];

        self.cpu.reset();
        self.cpu.load_program(&program);
        self.cpu.run(20);

        Ok(self.cpu.memory.read(result_addr))
    }

    /// Perform XOR using neural CPU
    fn neural_xor(&mut self, a: u8, b: u8) -> Result<u8, String> {
        let temp1 = self.get_temp_address();
        let temp2 = self.get_temp_address();
        let result_addr = self.get_temp_address();

        let program = vec![
            0xA, a,        // LDI a
            0x2, temp1,    // STORE temp1
            0xA, b,        // LDI b
            0x2, temp2,    // STORE temp2
            0x1, temp1,    // LOAD temp1
            0x7, temp2,    // XOR temp2 (neural XOR!)
            0x2, result_addr, // STORE result
            0xF,           // HALT
        ];

        self.cpu.reset();
        self.cpu.load_program(&program);
        self.cpu.run(20);

        Ok(self.cpu.memory.read(result_addr))
    }

    /// Get next available temporary memory address
    /// Use addresses 0xF, 0xE, 0xD working backwards (high memory)
    fn get_temp_address(&mut self) -> u8 {
        let addr = 0xF - self.memory_counter; // Start from 0xF and go down
        self.memory_counter = (self.memory_counter + 1) % 3; // Use addresses F, E, D
        addr
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple parser for basic syntax
pub struct Parser;

impl Parser {
    /// Parse a simple statement like "let x = 5 + 3"
    pub fn parse_statement(input: &str) -> Result<Statement, String> {
        let input = input.trim();
        
        if input.starts_with("let ") {
            Parser::parse_let(input)
        } else if input.starts_with("print(") && input.ends_with(')') {
            Parser::parse_print(input)
        } else if input.contains(" = ") {
            Parser::parse_assign(input)
        } else {
            Err(format!("Unknown statement: {}", input))
        }
    }

    /// Parse let statement: "let x = 5 + 3"
    fn parse_let(input: &str) -> Result<Statement, String> {
        let without_let = &input[4..]; // Remove "let "
        let parts: Vec<&str> = without_let.splitn(2, " = ").collect();
        
        if parts.len() != 2 {
            return Err("Invalid let statement. Use: let x = expression".to_string());
        }

        let var_name = parts[0].trim().to_string();
        let expr = Parser::parse_expression(parts[1])?;
        
        Ok(Statement::Let(var_name, expr))
    }

    /// Parse assignment: "x = 5 + 3"
    fn parse_assign(input: &str) -> Result<Statement, String> {
        let parts: Vec<&str> = input.splitn(2, " = ").collect();
        
        if parts.len() != 2 {
            return Err("Invalid assignment. Use: variable = expression".to_string());
        }

        let var_name = parts[0].trim().to_string();
        let expr = Parser::parse_expression(parts[1])?;
        
        Ok(Statement::Assign(var_name, expr))
    }

    /// Parse print statement: "print(x + 5)"
    fn parse_print(input: &str) -> Result<Statement, String> {
        let inner = &input[6..input.len()-1]; // Remove "print(" and ")"
        let expr = Parser::parse_expression(inner)?;
        Ok(Statement::Print(expr))
    }

    /// Parse expression: "5 + 3", "x", "a & b"
    /// Handles left-to-right evaluation: "a - b + c" becomes "(a - b) + c"
    fn parse_expression(input: &str) -> Result<Expr, String> {
        let input = input.trim();
        
        // Handle binary operations with proper left-to-right precedence
        // Find the rightmost operator of lowest precedence
        for op in &[" + ", " - "] {  // Addition/subtraction (lowest precedence)
            if let Some(pos) = input.rfind(op) {
                let left_str = &input[..pos];
                let right_str = &input[pos + op.len()..];
                
                let left = Box::new(Parser::parse_expression(left_str)?);
                let right = Box::new(Parser::parse_expression(right_str)?);
                
                return match op.trim() {
                    "+" => Ok(Expr::Add(left, right)),
                    "-" => Ok(Expr::Sub(left, right)),
                    _ => Err("Unknown operator".to_string()),
                };
            }
        }
        
        // Then handle logical operations (higher precedence)
        for op in &[" & ", " | ", " ^ "] {
            if let Some(pos) = input.rfind(op) {
                let left_str = &input[..pos];
                let right_str = &input[pos + op.len()..];
                
                let left = Box::new(Parser::parse_expression(left_str)?);
                let right = Box::new(Parser::parse_expression(right_str)?);
                
                return match op.trim() {
                    "&" => Ok(Expr::And(left, right)),
                    "|" => Ok(Expr::Or(left, right)),
                    "^" => Ok(Expr::Xor(left, right)),
                    _ => Err("Unknown operator".to_string()),
                };
            }
        }
        
        // Handle numbers
        if let Ok(num) = input.parse::<u8>() {
            return Ok(Expr::Number(num));
        }
        
        // Handle variables
        if input.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Ok(Expr::Variable(input.to_string()));
        }
        
        Err(format!("Invalid expression: {}", input))
    }
}

/// Test the interpreter
pub fn test() {
    println!("=== Neural Computer Interpreter Test ===");
    
    let mut interpreter = Interpreter::new();
    
    // Test 1: Simple arithmetic
    println!("\n--- Test 1: Neural Addition ---");
    let program1 = vec![
        "let a = 5",
        "let b = 3", 
        "let result = a + b",
        "print(result)"
    ];
    
    run_program(&mut interpreter, program1);
    
    // Test 2: Logical operations
    println!("\n--- Test 2: Neural Logic ---");
    let program2 = vec![
        "let x = 12",  // 1100 in binary
        "let y = 5",   // 0101 in binary
        "let and_result = x & y",
        "let or_result = x | y", 
        "let xor_result = x ^ y",
        "print(and_result)",
        "print(or_result)",
        "print(xor_result)"
    ];
    
    run_program(&mut interpreter, program2);
    
    // Test 3: Complex expression
    println!("\n--- Test 3: Complex Expression ---");
    let program3 = vec![
        "let a = 8",
        "let b = 3",
        "let c = 2", 
        "let result = a - b + c",
        "print(result)"
    ];
    
    run_program(&mut interpreter, program3);
}

/// Helper function to run a program
fn run_program(interpreter: &mut Interpreter, lines: Vec<&str>) {
    let mut statements = Vec::new();
    
    for line in lines {
        match Parser::parse_statement(line) {
            Ok(stmt) => {
                println!("  {}", line);
                statements.push(stmt);
            }
            Err(e) => {
                println!("Parse error in '{}': {}", line, e);
                return;
            }
        }
    }
    
    match interpreter.execute(statements) {
        Ok(()) => println!("✓ Program executed successfully"),
        Err(e) => println!("✗ Execution error: {}", e),
    }
}

