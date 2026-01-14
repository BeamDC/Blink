const std = @import("std");
const Token = @import("tokenization/token.zig").Token;
const lexer = @import("tokenization/lexer.zig").lexer;
const zbench = @import("zbench");

pub fn main() !void {
    var gpa: std.heap.DebugAllocator(.{}) = .init;
    defer _ = gpa.deinit();
    const alloc = gpa.allocator();

    // read the test data
    const test_path = "test_data.txt";
    const file = try std.fs.cwd().openFile(test_path, .{});
    defer file.close();

    const stat = try file.stat();
    const file_size: usize = @intCast(stat.size);

    const content: []const u8 = try file.readToEndAlloc(alloc, file_size);
    defer alloc.free(content);

    // construct the lexer
    var lex = lexer.init(content);

    // tokenize
    var timer = try std.time.Timer.start();
    const tokens: []Token = try alloc.alloc(Token, lex.src.len / 2 + 1);
    const num_tokens = try lex.tokenize(tokens);
    defer alloc.free(tokens);
    const elapsed: f64 = @floatFromInt(timer.read());

    for (tokens[0..num_tokens]) |token| {
        std.debug.print("{f}\n", .{token});
    }

    std.debug.print("tokenized in: {d:.2} microseconds\n", .{elapsed / 1000});
    std.debug.print("{d} tokens used, {d} available\n", .{num_tokens + 1, tokens.len});
}