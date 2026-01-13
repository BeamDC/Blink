const std = @import("std");
const lexer = @import("tokenization/lexer.zig").lexer;

pub fn main() !void {
    var gpa: std.heap.DebugAllocator(.{}) = .init;
    const alloc = gpa.allocator();

    var lex = lexer.init(
        \\fn main()  {
        \\  const pi = 3.14159265;
        \\}
    );
    // std.debug.print("Lexer:\n{any}\n", .{lex});

    var timer = try std.time.Timer.start();
    const tokens = try lex.tokenize(alloc);
    const elapsed: f64 = @floatFromInt(timer.read());

    for (tokens) |token| {
        std.debug.print("{f}\n", .{token});
    }

    std.debug.print("tokenized in: {d:.2} microseconds\n", .{elapsed / 1000});
}