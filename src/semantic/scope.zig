const std = @import("std");
const Symbol = @import("symbol.zig").Symbol;
const Type = @import("symbol.zig").Type;

pub const Scope = struct {
    parent: ?*Scope,
    symbols: std.StringHashMap(Symbol),
};