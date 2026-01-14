pub const std = @import("std");
pub const Token = @import("../tokenization/token.zig").Token;

pub const Ast = struct {
    nodes: []AstNode,
    extra: []u32,
};

pub const AstNode = struct {
    tag: Tag,
    token: Token,
    data: Data,

    pub const Tag = enum(u8) {
        root,

        // logical / comparison operators
        eq,
        neq,
        gt,
        lt,
        geq,
        leq,

        // mathematical operators
        add,
        sub,
        mul,
        div,
        mod,

        // todo: bitwise operators

        // control flow
        @"const",
        @"if",
        @"fn",
        call,
        ret,
    };

    pub const Data = struct {
        left: u32,
        right: u32,
    };
};