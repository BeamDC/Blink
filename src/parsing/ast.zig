pub const std = @import("std");
pub const Token = @import("../tokenization/token.zig").Token;

pub const Ast = struct {
    alloc: std.mem.Allocator,
    nodes: []AstNode,
    data: []u32,

    /// initialize a new Ast with space for n nodes
    pub fn init(n: usize, alloc: std.mem.Allocator) !Ast {
        return Ast {
            .nodes = try alloc.alloc(AstNode, n),
            .alloc = alloc,
        };
    }

    pub fn deinit(self: *Ast) void {
        self.alloc.free(self.nodes);
    }
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
        start: u32,
        end: u32,
    };
};

