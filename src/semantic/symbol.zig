const AstNode = @import("../parsing/ast.zig").AstNode;

pub const Type = union(enum) {
    primitive: PrimitiveType,
    @"struct": StructType,
    function: FunctionType,

    pub const PrimitiveType = enum {
        void,
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
};

pub const Symbol = struct {
    kind: SymbolKind,
    type: *Type,
    node: *AstNode,

    pub const SymbolKind = enum {
        variable,
        function,
        type,
    };
};