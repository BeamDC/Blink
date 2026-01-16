const std = @import("std");
const Token = @import("../tokenization/token.zig").Token;
const TokenType = @import("../tokenization/token.zig").TokenType;
const Ast = @import("ast.zig").Ast;
const ParseError = @import("error.zig").ParseError;


//https://github.com/ziglang/zig/blob/master/lib/std/zig/Parse.zig#L192

/// the parser
pub const Parser = @This();
/// the allocator used by the parser
alloc: std.mem.Allocator,
/// the list of tokens to be parsed
tokens: []Token,
/// the abstract syntax tree
ast: Ast,
/// the number of tokens in the token list
num_tokens: usize,
/// the current position in the token list
pos: usize,

pub fn init(tokens: []Token, num_tokens: usize, alloc: std.mem.Allocator) Parser {
    return Parser {
        .alloc = alloc,
        .tokens = tokens,
        .ast = Ast.init(num_tokens, alloc),
        .num_tokens = num_tokens,
        .pos = 0,
    };
}

/// return the next token in the stream, and advance the parser's position.
fn next(self: *Parser) !Token {
    _ = self;
    return ParseError.UnexpectedToken;
}

/// expect the given token type, if it is not next in the token stream and
/// error is returned, if it is, its consumed
fn expect(self: *Parser, ttype: TokenType) !void {
    _ = self;
    _ = ttype;
    return ParseError.UnexpectedToken;
}

/// parse the root file of a project, returning any errors encountered.
pub fn parse_root(self: *Parser) !void {
    _ = self;
    return ParseError.UnexpectedToken;
}

/// parse a function definition from the token stream
pub fn parse_fn(self: *Parser) !usize {
    _ = self;
    return ParseError.UnexpectedToken;
}

/// parse an expression from the token stream
pub fn parse_expr(self: *Parser) !usize {
    _ = self;
    return ParseError.UnexpectedToken;
}
