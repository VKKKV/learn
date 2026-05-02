const std = @import("std");

const FileError = error{ AccessDenied, NotFound };

fn readNumber()!u32 {
    return FileError.NotFound;
}

test "allocation" {
    const allocator = std.heap.page_allocator;
    const memory = try allocator.alloc(u8, 100);
    defer allocator.free(memory);
}

fn List(comptime T: type) type {
    return struct {
        items: T,
        len: usize,
    };
}

pub fn main() !void {
    const a: i32 = 20;
    var b = @as(u32, a);
    b += 5;

    const name = "Zig";
    std.debug.print("Hello, {s}!\n", .{name});

    var array = [_]u8{ 1, 2, 3, 4, 5 };

    const ptr = &array;
    const multi_ptr: [*]u8 = &array;
    const slice: []u8 = array[1..4];

    const num = readNumber() catch 42;
    const num2 = try readNumber();

    // errdefer 演示
    // const buf = try allocator.alloc(u8, 1024);
    // errdefer allocator.free(buf);

    const my_list = List(i32){
        .items = undefined,
        .len = 0,
    };

    _ = ptr;
    _ = multi_ptr;
    _ = slice;
    _ = num;
    _ = num2;
    _ = my_list;
}
