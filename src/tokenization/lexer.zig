const token = @import("token.zig");

pub const lexer = struct {
    src: []const u8,
    start: usize,
    current: usize,

    /// Attempts to return the next token from the input stream,
    /// will return any errors that are encountered
    fn nextToken(lex: *lexer) !token {
        @panic("TODO: return the next token from the input stream");
    }

    /// Tokenize the entire input stream, returning the slice of tokens
    pub fn tokenize(lex: *lexer) ![]const token {
        @panic("TODO: tokenie the full input stream");
    }
};