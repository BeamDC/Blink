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
    param: Param,
    param_list: ParamList,

    // expressions
    type: TypeExpr,
    block: Block,
    literal: Literal,
    ident: Ident,
    call: FnCall,
    unary: UnOp,
    binary: BinOp,

    /// the root node of a file, contains pointers to all top level nodes.
    pub const Root = struct {
        nodes: []const *AstNode,
    };

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
        @"else": ?*AstNode,
    };

    pub const RetStmt = struct {
        value: *const AstNode,
    };

    pub const FnStmt = struct {
        name: Token,
        params: *AstNode,
        ret: *AstNode,
        body: *AstNode,
    };

    pub const Param = struct {
        name: Token,
        type: *AstNode,
    };

    pub const ParamList = struct {
        params: []const *AstNode,
    };

    pub const TypeExpr = struct {
        name: Token,
        nullable: bool,
    };

    pub const Block = struct {
        statements: []const *AstNode,
    };

    pub const Literal = struct {
        val: Token,
    };

    pub const Ident = struct {
        name: Token,
    };

    pub const FnCall = struct {
        name: Token,
        args: []const *AstNode,
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

    pub fn format(self: AstNode, writer: *std.io.Writer) !void {
        switch (self) {
            .root => |r| {
                for (r.nodes) |node| {
                    try writer.print("{f}\n", .{node.*});
                }
            },
            .@"const" => |c| try writer.print("const {s} = {f}", .{c.name.raw, c.value}),
            .literal => |l| try writer.print("{s}", .{l.val.raw}),
            .ident => |i| try writer.print("{s}", .{i.name.raw}),
            .unary => |u| try writer.print("{s}({f})", .{@tagName(u.op.type), u.operand.*}),
            .binary => |b| try writer.print("{s}({f}, {f})", .{@tagName(b.op.type), b.left.*, b.right.*}),
            else => try writer.print("TODO: impl format for {s}", .{@tagName(self)}),
        }
    }
};