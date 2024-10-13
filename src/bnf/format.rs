use super::grammar::Grammar;
use std::fmt::Write;

pub trait Format {
    fn format(&self, output: &mut dyn Write, grammar: &Grammar) -> std::fmt::Result;
}
