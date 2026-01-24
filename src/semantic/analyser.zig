const std = @import("std");
const Scope = @import("scope.zig").Scope;
const AstNode = @import("../parsing/ast.zig").AstNode;

pub const SemanticAnalyser = struct {
    alloc: std.mem.Allocator,
    scopes: std.ArrayList(*Scope),

    /// two pass semantic analysis over all of the top level nodes
    pub fn analyse(self: *SemanticAnalyser, nodes: []const *AstNode) !void {
        _ = self;
        _ = nodes;
    }

    /// analyze top level nodes to build context on types, functions, etc
    fn analyzeDecl(self: *AstNode, node: *AstNode) !void {
        _ = self;
        _ = node;
    }

    /// recursively analyzes a node of the ast
    fn analyzeNode(self: *SemanticAnalyser, node: *AstNode) !void {
        _ = self;
        _ = node;
    }
};