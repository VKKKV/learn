//
// Great news! Now we know enough to understand a "real" Hello World
// program in Zig - one that uses the system Standard Out resource...which
// can fail!
//
const std = @import("std");

// Take note that this main() definition now returns "!void" rather
// than just "void". Since there's no specific error type, this means
// that Zig will infer the error type. This is appropriate in the case
// of main(), but can make a function harder (function pointers) or
// even impossible to work with (recursion) in some situations.
//
// You can find more information at:
// https://ziglang.org/documentation/master/#Inferred-Error-Sets
//
// pub fn main(init: std.process.Init) !void {
// 0.15+
pub fn main() !void {
    // 1. Get the stdout file handle
    const file = std.fs.File.stdout();

    // 2. Create a writer with a buffer (e.g., 1024 bytes)
    // Buffered by Default: The new I/O subsystem requires a buffer to minimize costly system calls. If you do not want a buffer, you can pass an empty slice (&.{}), but this is slower.
    // var buffer: [1024]u8 = undefined;
    var stdout_writer = file.writer(&.{});

    // 3. Access the interface and print
    const stdout = &stdout_writer.interface;
    try stdout.print("Hello world!\n", .{});
}
