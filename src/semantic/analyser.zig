const std = @import("std");
const Scope = @import("scope.zig").Scope;
const AstNode = @import("../parsing/ast.zig").AstNode;
const SemanticError = @import("error.zig").SemanticError;
const Symbol = @import("symbol.zig").Symbol;
const Type = @import("symbol.zig").Type;

pub const SemanticAnalyser = @This();

alloc: std.mem.Allocator,

/// the global scope
global: *Scope,

/// a stack of scopes, will contain all currently available scopes
/// from the current scope during the second pass of analysis.
stack: std.ArrayList(*Scope),

pub fn init(alloc: std.mem.Allocator) !SemanticAnalyser {
    const global = try alloc.create(Scope);
    global.* = Scope{
        .parent = null,
        .symbols = std.StringHashMap(Symbol).init(alloc),
    };

    return SemanticAnalyser{
        .alloc = alloc,
        .global = global,
        .stack = try std.ArrayList(*Scope).initCapacity(alloc, 1),
    };
}

/// two pass semantic analysis over all of the top level nodes,
/// expects the given node to be a file root
pub fn analyse(self: *SemanticAnalyser, root: *AstNode) !void {
    if (root.* != .root) { return SemanticError.ExpectedRoot; }

    // first pass
    for (root.*.root.nodes) |node| {
        try self.analyzeDecl(node);
    }

    // second pass
    // for (root.*.root.nodes) |node| {
        // try self.analyzeNode(node);
    // }
}

/// analyze top level nodes to build context on types, functions, etc
fn analyzeDecl(self: *SemanticAnalyser, node: *AstNode) !void {
    switch (node.*) {
        .@"const" => |n| {
            const symbol = Symbol {
                .kind = .constant,
                .type = try Type.create(self.alloc, try infer(n.value)),
                .node = node,
            };
            try self.global.insert(n.name.raw, symbol);
        },
        .@"fn" => |_| {},
        else => return,
    }
}

/// recursively analyzes a node of the ast, performing full semantic analysis
fn analyzeNode(self: *SemanticAnalyser, node: *AstNode) !void {
    _ = self;
    _ = node;
}

/// infers the type of some node
fn infer(node: *AstNode) !Type {
    return switch (node.*) {
        // todo : actually infer the type, we default to void for now
        else => .{ .primitive = .void },
    };
}