const std = @import("std");
const Token = @import("../tokenization/token.zig").Token;
const Ast = @import("ast.zig").Ast;
const ParseError = @import("error.zig").ParseError;
// const Allocator = std.mem.Allocator;

//https://github.com/ziglang/zig/blob/master/lib/std/zig/Parse.zig#L192
pub const Parser = struct {
    /// the list of tokens to be parsed
    tokens: []Token,
    /// the number of tokens in the token list
    num_tokens: usize,
    /// the current position in the token list
    pos: usize = 0,

    pub fn init(tokens: []Token, num_tokens: usize) Parser {
        return Parser {
            .tokens = tokens,
            .num_tokens = num_tokens,
        };
    }

    /// parse the root file of a project, returning the Ast, or any errors encountered.
    pub fn parse_root(self: *Parser) !Ast {
        _ = self;
        return ParseError.UnexpectedToken;
    }
};