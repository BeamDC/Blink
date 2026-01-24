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

    fn writeIndent(writer: *std.io.Writer, i: usize) !void {
        for (0..i) |_| {
            try writer.print("  ", .{});
        }
    }

    fn formatIndented(self: AstNode, writer: *std.io.Writer, indent: usize) !void {
        switch (self) {
            .root => |r| {
                for (r.nodes) |node| {
                    try node.formatIndented(writer, indent);
                    try writer.writeByte('\n');
                }
            },
            .@"const" => |c| {
                try writeIndent(writer, indent);
                try writer.print("const {s} = {f}", .{c.name.raw, c.value});
            },
            .let => |l| {
                try writeIndent(writer, indent);
                try writer.print("let {s} = {f}", .{l.name.raw, l.value});
            },
            .@"if" => |i| {
                try writeIndent(writer, indent);
                try writer.print("if ", .{});
                try i.clause.formatIndented(writer, indent);
                try writer.print(" then:\n", .{});

                try i.then.formatIndented(writer, indent);

                if (i.@"else") |else_block| {
                    try writer.writeByte('\n');
                    try writeIndent(writer, indent);
                    try writer.print("else:\n", .{});
                    try else_block.formatIndented(writer, indent);
                }
            },
            .ret => |r| {
                try writeIndent(writer, indent);
                try writer.print("ret {f}", .{r.value});
            },
            .@"fn" => |f| try writer.print("fn {s}({f}) -> {f} {{\n{f}\n}}", .{f.name.raw, f.params, f.ret, f.body}),
            .param => |p| try writer.print("{s}: {f}", .{p.name.raw, p.type}),
            .param_list => |p| {
                for (p.params, 0..) |param, i| {
                    if (i != p.params.len - 1) {
                        try writer.print(", ", .{});
                    }
                    try writer.print("{f}", .{param});
                }
            },
            .type => |t| {
                if (t.nullable) {
                    try writer.print("?", .{});
                }
                try writer.print("{s}", .{t.name.raw});
            },
            .block => |b| {
                for (b.statements, 0..) |stmt, i| {
                    if (i != 0) {
                        try writer.writeByte('\n');
                    }
                    try stmt.formatIndented(writer, indent + 1);
                }
            },
            .literal => |l| {
                try writeIndent(writer, indent);
                try writer.print("{s}", .{l.val.raw});
            },
            .ident => |i| {
                try writeIndent(writer, indent);
                try writer.print("{s}", .{i.name.raw});
            },
            .call => |c| {
                try writeIndent(writer, indent);
                try writer.print("{s}(", .{c.name.raw});
                for (c.args, 0..) |arg, i| {
                    try writer.print("{f}", .{arg});
                    if (i != c.args.len - 1) {
                        try writer.print(", ", .{});
                    }
                }
                try writer.print(")", .{});
            },
            .unary => |u| try writer.print("{s}({f})", .{@tagName(u.op.type), u.operand}),
            .binary => |b| try writer.print("{s}({f}, {f})", .{@tagName(b.op.type), b.left, b.right}),
        // else => try writer.print("TODO: impl format for {s}", .{@tagName(self)}),
        }
    }

    pub fn format(self: AstNode, writer: *std.io.Writer) !void {
        try self.formatIndented(writer, 0);
    }
};