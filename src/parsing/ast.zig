pub const std = @import("std");
pub const Token = @import("../tokenization/token.zig").Token;
pub const TokenType = @import("../tokenization/token.zig").TokenType;

pub const AstNode = union(enum) {
    root: Root,

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

    /// the root node of a file, contains pointers to all top level nodes.
    pub const Root = struct {
        nodes: []const AstNode,
    };

    pub const ConstStmt = struct {
        name: Token,
        value: *const AstNode,
    };

    pub const LetStmt = struct {
        name: Token,
        value: *const AstNode,
    };

    pub const IfStmt = struct {
        clause: *const AstNode,
        then: *const AstNode,
        @"else": *const AstNode,
    };

    pub const RetStmt = struct {
        value: *const AstNode,
    };

    pub const FnStmt = struct {
        name: Token,
        params: []const AstNode,
        body: *const AstNode,
    };

    pub const TypeExpr = struct {
        name: Token,
        nullable: bool,
    };

    pub const Block = struct {
        statements: []const AstNode,
    };

    pub const UnOp = struct {
        op: Token,
        operand: *const AstNode,
    };

    pub const BinOp = struct {
        op: Token,
        left: *const AstNode,
        right: *const AstNode,
    };

    pub fn format(self: AstNode, writer: *std.io.Writer) !void {
        try writer.print("{s}", .{@tagName(self)});
    }
};