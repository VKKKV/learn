--
--[[ another comment
-- This is a multi-line comment
--
--
--]]

print("Hello Lua")
-- concatenation operator
print("Hello" .. " " .. "Lua")

print(10 + 5)
print("\n")

-- strings
aa = [[
aa
bb
]]
print(aa)
aa = [[ cc dd ]]
print(aa)

-- arithmetic operators
print("2" + 6)
print("2" + "6")
print("2 + 6")
print("-2e2" * "6")
-- print("error" + 1)

-- length operator
print(#"www.runoob.com")

-- indexing starts at 1
local tbl = { "apple", "pear", "orange", "grape" }
for key, value in pairs(tbl) do
    print(key, value)
end

-- functions
function foo(a, b)
    return a + b, a - b
end

print("Function return values:")
print(foo(10, 5))
print(type(foo(10, 5)))

-- date type
print(type("Hello world")) --> string
print(type(10.4 * 3)) --> number
print(type(print)) --> function
print(type(true)) --> boolean
print(type(nil)) --> nil
print(type(X)) --> nil
print(type(type(X))) --> string

print(type(X) == "nil")
print(type(X) == nil)
