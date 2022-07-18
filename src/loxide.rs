use crate::scanner::Scanner;
use crate::token::Token;
use anyhow::Result;
use std::fs::read;
use std::io;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};

static HAD_ERROR: AtomicBool = AtomicBool::new(false);

pub struct Loxide {
    had_error: bool,
}

impl Loxide {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn run_file(&self, path: &str) -> Result<()> {
        self.run(&String::from_utf8(read(&path)?)?);
        Ok(())
    }

    pub fn run_prompt(&self) -> Result<()> {
        let mut stdout = io::stdout();
        let stdin = io::stdin();
        let mut input = String::new();
        loop {
            write!(stdout, "> ")?;
            stdout.flush()?;
            stdin.read_line(&mut input)?;
            self.run(&input);
            Loxide::set_had_error(false);
            input.clear();
        }
    }

    pub fn run(&self, source: &str) {
        let mut scanner = Scanner::new(source.to_owned());
        let tokens: Vec<Token> = scanner.scan_tokens();

        for token in tokens {
            println!("{}", token);
        }
    }

    pub fn error(line: usize, message: &str) {
        Self::report(line, "", message);
    }

    fn report(line: usize, where_: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, where_, message);
        Self::set_had_error(true);
    }

    pub fn set_had_error(errored: bool) {
        HAD_ERROR.store(errored, Ordering::Relaxed)
    }
}

impl Default for Loxide {
    fn default() -> Self {
        Self::new()
    }
}
