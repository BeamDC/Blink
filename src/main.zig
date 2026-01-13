const std = @import("std");
const lexer = @import("tokenization/lexer.zig").lexer;

pub fn main() !void {
    // var gpa: std.heap.DebugAllocator(.{}) = .init;
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

    var lex = lexer.init(content);
    // std.debug.print("Lexer:\n{any}\n", .{lex});

    var timer = try std.time.Timer.start();
    const tokens = try lex.tokenize(alloc);
    defer alloc.free(tokens);
    const elapsed: f64 = @floatFromInt(timer.read());

    for (tokens) |token| {
        std.debug.print("{f}\n", .{token});
    }

    std.debug.print("tokenized in: {d:.2} microseconds\n", .{elapsed / 1000});
}