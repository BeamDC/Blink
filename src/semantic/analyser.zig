const std = @import("std");
const Scope = @import("scope.zig").Scope;
const AstNode = @import("../parsing/ast.zig").AstNode;

pub const SemanticAnalyser = struct {
    alloc: std.mem.Allocator,
    scopes: std.ArrayList(*Scope),

    pub fn init(alloc: std.mem.Allocator) !SemanticAnalyser {
        return SemanticAnalyser {
            .alloc = alloc,
            .scopes = try std.ArrayList(*Scope).initCapacity(alloc, 1),
        };
    }

    /// two pass semantic analysis over all of the top level nodes,
    /// expects the given node to be a file root
    pub fn analyse(self: *SemanticAnalyser, nodes: *AstNode) !void {
        _ = self;
        _ = nodes;
    }

    /// analyze top level nodes to build context on types, functions, etc
    fn analyzeDecl(self: *SemanticAnalyser, node: *AstNode) !void {
        _ = self;
        switch (node.*) {
            .@"const" => |_| {},
            .@"fn" => |_| {},
            else => return,
        }
    }

    /// recursively analyzes a node of the ast
    fn analyzeNode(self: *SemanticAnalyser, node: *AstNode) !void {
        _ = self;
        _ = node;
    }
};