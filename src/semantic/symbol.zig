const std = @import("std");
const AstNode = @import("../parsing/ast.zig").AstNode;

pub const Type = union(enum) {
    primitive: PrimitiveType,
    @"struct": StructType,
    function: FunctionType,
    optional: Optional,
    unresolved: Unresolved,

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
        @"return": *const Type,
    };

    pub const Optional = struct {
        inner: *Type,
    };

    pub const Unresolved = union(enum) {
        named: Named,
        Unknown,

        pub const Named = struct {
            name: []const u8,
        };

        pub fn format(self: Unresolved, writer: *std.io.Writer) !void {
            switch (self) {
                .named => |u| try writer.print("Named({s})", .{u.name}),
                .Unknown => try writer.print("Unknown", .{}),
            }
        }
    };

    pub fn create(alloc: std.mem.Allocator, ty: Type) !*Type {
        const ptr = try alloc.create(Type);
        ptr.* = ty;
        return ptr;
    }

    pub fn format(self: Type, writer: *std.io.Writer) !void {
        switch (self) {
            .@"struct" => try writer.print("TODO : FORMAT STRUCT", .{}),
            .function => |t| {
                try writer.print("(", .{});
                for (t.params, 0..) |param, i| {
                    try writer.print("{f}", .{param});
                    if (i != t.params.len - 1) {
                        try writer.print(", ", .{});
                    }
                }
                try writer.print(") {f}", .{t.@"return"});
            },
            .primitive => |t| try writer.print("{s}", .{ @tagName(t) }),
            .optional => |t| try writer.print("Optional({f})", .{t.inner}),
            .unresolved => |t| try writer.print("{f}", .{t}),
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