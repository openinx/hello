k, b = collectgarbage("count")
print(k)
print(b)
assert(k*1024 == math.floor(k)*1024 + b)

a = {}

for i=1,10 do
    local y = 0 
    -- a[i] = function() y=y+1; return x + y end
    a[i] = i * 2
end

for i,v in ipairs(a) do 
    print(string.format('(%d, %d)', i, v))
end

local t = {name="lua", version="5.2"}
x = string.gsub("$name-$version.tar.gz", "%$(%w+)", t)
print(x)


io.write('hello world, I love programming.\n')

