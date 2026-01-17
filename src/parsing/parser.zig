const std = @import("std");
const Token = @import("../tokenization/token.zig").Token;
const TokenType = @import("../tokenization/token.zig").TokenType;
const Ast = @import("ast.zig").Ast;
const ParseError = @import("error.zig").ParseError;
const Tag = @import("ast.zig").AstNode.Tag;

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
        .ast = Ast.init(num_tokens / 2, alloc) catch {
            @panic("ast alloc failed");
        },
        .num_tokens = num_tokens,
        .pos = 0,
    };
}

pub fn deinit(self: *Parser) void {
    self.ast.deinit();
}

fn end(self: *Parser) bool {
    return self.pos >= self.num_tokens;
}

/// return the next token in the stream if it exists, and advance the parser's position.
fn advance(self: *Parser) Token {
    const tok = self.tokens[self.pos];
    self.pos += 1;
    return tok;
}

/// return the next token in the stream if it exists, without advancing the parser's position.
fn peek(self: *Parser) ?Token {
    if (self.end()) return null;
    return self.tokens[self.pos];
}

/// expect the given token type, if it is not next in the token stream and
/// error is returned, if it is, its consumed and returned
fn expect(self: *Parser, ttype: TokenType) !Token {
    const peeked = self.peek();
    if (peeked == null) return ParseError.UnexpectedEndOfTokens;
    if (peeked.?.type != ttype) return ParseError.UnexpectedToken;
    return self.advance();
}

/// parse the root file of a project, returning the number of nodes used or any errors encountered.
pub fn parse_root(self: *Parser) !usize {
    var i: usize = 0;
    while (!self.end()) : (i += 1) {
        const first = self.advance();
        switch (first.type) {
            .Fn => _ = try self.parse_fn(),
            .Const => _ = try self.parse_const(),
            else => return ParseError.UnexpectedToken,
        }
    }
    return i;
}

/// parse a function definition from the token stream
fn parse_fn(self: *Parser) !usize {
    _ = self;
    return ParseError.UnexpectedToken;
}

/// parse a constant definition from the token stream
fn parse_const(self: *Parser) !usize {
    // wip
    // const name = try self.expect(TokenType.Ident);
    // _ = try self.expect(TokenType.Assign);
    // const expr = self.parse_expr();
    // _ = try self.expect(TokenType.Semicolon);
    _ = self;
    return ParseError.UnexpectedToken;
}

/// parse a let statement from the token stream
fn parse_let(self: Parser) !usize {
    _ = self;
    return ParseError.UnexpectedToken;
}

/// parse an if statement from the token stream
fn parse_if(self: Parser) !usize {
    _ = self;
    return ParseError.UnexpectedToken;
}

/// parse a return statement from the token stream
fn parse_ret(self: Parser) !usize {
    _ = self;
    return ParseError.UnexpectedToken;
}

/// parse an expression from the token stream
fn parse_expr(self: *Parser) !usize {
    _ = self;
    return ParseError.UnexpectedToken;
}
