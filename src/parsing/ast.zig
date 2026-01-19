pub const std = @import("std");
pub const Token = @import("../tokenization/token.zig").Token;
pub const TokenType = @import("../tokenization/token.zig").TokenType;

pub const AstNode = union(enum) {
    // statements
    @"const": ConstStmt,
    let: LetStmt,
    @"if": IfStmt,
    ret: RetStmt,
    @"fn": FnStmt,

    // expressions
    type: TypeExpr,
    block: Block,
    unary: UnOp,
    binary: BinOp,

    pub const ConstStmt = struct {
        name: Token,
        value: *AstNode,
    };

    pub const LetStmt = struct {
        name: Token,
        value: *AstNode,
    };

    pub const IfStmt = struct {
        clause: *AstNode,
        then: *AstNode,
        @"else": *AstNode,
    };

    pub const RetStmt = struct {
        value: *AstNode,
    };

    pub const FnStmt = struct {
        name: Token,
        params: []const *AstNode,
        body: *AstNode,
    };

    pub const TypeExpr = struct {
        name: Token,
        nullable: bool,
    };

    pub const Block = struct {
        statements: []const *AstNode,
    };

    pub const UnOp = struct {
        op: Token,
        operand: *AstNode,
    };

    pub const BinOp = struct {
        op: Token,
        left: *AstNode,
        right: *AstNode,
    };
};