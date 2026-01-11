const std = @import("std");
const tokenization = @import("tokenization/lexer.zig");

pub fn main() !void {
    var lex = tokenization.lexer {
        .src = "",
        .start = 0,
        .current = 0,
    };
    std.debug.print("Lexer:\n{any}", .{lex});

    const tokens = try lex.tokenize();
    std.debug.print("Tokens:\n{any}", .{tokens});
}