const std = @import("std");
const AstNode = @import("../parsing/ast.zig").AstNode;

pub const Type = union(enum) {
    primitive: PrimitiveType,
    @"struct": StructType,
    function: FunctionType,

    pub const PrimitiveType = enum {
        void,
        // todo : arbitrary precision & comptime ints instead of standard ints
        i32,
        u32,
        f32,
    };

    pub const StructType = struct {
        // todo : structs are not supported yet
    };

    pub const FunctionType = struct {
        params: []const *Type,
        @"return": *Type,
    };

    pub fn create(alloc: std.mem.Allocator, ty: Type) !*Type {
        const ptr = try alloc.create(Type);
        ptr.* = ty;
        return ptr;
    }

    pub fn format(self: Type, writer: *std.io.Writer) !void {
        switch (self) {
            .@"struct" => try writer.print("TODO : FORMAT STRUCT", .{}),
            .function => try writer.print("TODO FORMAT FUNCTION", .{}),
            .primitive => try writer.print("{s}", .{ @tagName(self.primitive) }),
        }
    }
};

pub const Symbol = struct {
    kind: SymbolKind,
    type: *Type,
    node: *AstNode,

    pub const SymbolKind = enum {
        constant,
        variable,
        function,
        type,
    };

    pub fn format(self: Symbol, writer: *std.io.Writer) !void {
        try writer.print("{s} {f}", .{ @tagName(self.kind), self.type.* });
    }
};