const std = @import("std");
const Token = @import("../tokenization/token.zig").Token;
const TokenType = @import("../tokenization/token.zig").TokenType;
const AstNode = @import("ast.zig").AstNode;
const ParseError = @import("error.zig").ParseError;

/// the parser
pub const Parser = @This();
/// the allocator used by the parser
alloc: std.mem.Allocator,
/// the list of tokens to be parsed
tokens: []Token,
/// the number of tokens in the token list
num_tokens: usize,
/// the current position in the token list
pos: usize,

pub fn init(tokens: []Token, num_tokens: usize, alloc: std.mem.Allocator) Parser {
    return Parser {
        .alloc = alloc,
        .tokens = tokens,
        .num_tokens = num_tokens,
        .pos = 0,
    };
}

inline fn end(self: *Parser) bool {
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

/// returns the precedence of a binary operator.
fn binary_precedence(ttype: TokenType) ?u8 {
    return switch (ttype) {
        .Assign, .CompAdd, .CompSub,
        .CompMul, .CompDiv, .CompMod,
        .CompBitNot, .CompBitOr, .CompBitXor,
        .CompBitAnd, .CompLshift, .CompRshift => 1,
        .Or => 2,
        .And => 3,
        .BitOr => 4,
        .BitXor => 5,
        .BitAnd => 6,
        .Eq, .Neq => 7,
        .Gt, .Ge, .Lt, .Le, => 8,
        .Lshift, .Rshift => 9,
        .Add, .Sub => 10,
        .Mul, .Div, .Mod => 11,
        .Dot => 99, // temp value, but this must be the max lol
        else => null,
    };
}

/// parse the root file of a project, writing the parsed nodes to the given ArrayList.
pub fn parse_root(self: *Parser) !AstNode {
    var nodes = try std.ArrayList(AstNode).initCapacity(self.alloc, self.num_tokens / 3);
    defer nodes.deinit(self.alloc);

    // parse until no tokens remain
    while (!self.end()) : (self.pos += 1) {
        switch (self.tokens[self.pos].type) {
            .Const => try nodes.append(self.alloc, try self.parse_const()),
            else => return ParseError.UnexpectedToken,
        }
    }

    return AstNode {
        .root = AstNode.Root {
            .nodes = try nodes.toOwnedSlice(self.alloc)
        }
    };
}

/// parse a function definition from the token stream
fn parse_fn(self: *Parser) !AstNode {
    _ = self;
    return ParseError.UnexpectedToken;
}

/// parse a constant definition from the token stream
fn parse_const(self: *Parser) !AstNode {
    _ = try self.expect(TokenType.Const);
    const name = try self.expect(TokenType.Ident);
    _ = try self.expect(TokenType.Assign);
    const expr = try self.parse_expr();
    _ = try self.expect(TokenType.Semicolon);

    return AstNode {
        .@"const" = AstNode.ConstStmt {
            .name = name,
            .value = &expr,
        }
    };
}

/// parse a let statement from the token stream
fn parse_let(self: Parser) !AstNode {
    _ = try self.expect(TokenType.Let);
    const name = try self.expect(TokenType.Ident);
    _ = try self.expect(TokenType.Assign);
    const expr = try self.parse_expr();
    _ = try self.expect(TokenType.Semicolon);

    return AstNode {
        .let = AstNode.LetStmt {
            .name = name,
            .value = &expr,
        }
    };
}

/// parse an if statement from the token stream
fn parse_if(self: Parser) !AstNode {
    _ = self;
    return ParseError.UnexpectedToken;
}

/// parse a return statement from the token stream
fn parse_ret(self: Parser) !AstNode {
    _ = try self.expect(TokenType.Ret);
    const expr = try self.parse_expr();
    _ = try self.expect(TokenType.Semicolon);

    return AstNode {
        .ret = AstNode.RetStmt {
            .value = &expr,
        }
    };
}

/// parse an expression from the token stream
fn parse_expr(self: *Parser) !AstNode {
    _ = self;
    return ParseError.UnexpectedToken;
}

/// parse a block from the token stream
fn parse_block(self: *Parser) !AstNode {
    _ = self;
    return ParseError.UnexpectedToken;
}