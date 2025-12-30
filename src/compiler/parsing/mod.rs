use crate::compiler::CompileError;
use crate::compiler::tokenization::token::{TokenStream};

pub mod ast;
pub mod expr;
pub mod stmt;
mod type_expr;
/* The grammar will consist of two main parts, expressions and statements.
 * Expressions:
 * | - return a value
 * | - ordered (are dependent on other chained expressions)
 * | - 
 * : assignment, loops, if/match, math, ...
 *
 * Statements:
 * | - return nothing
 * | - unordered (can be used as long as they are defined somewhere)
 * : function definitions, struct/enums, traits, impl blocks, ...
 */

pub trait Parse<'p>: Sized {
    /// parse `self` out of the given token stream
    fn parse(stream: &mut TokenStream<'p>) -> Result<Self, CompileError>;
}