-- usr/local/bin/lua

function fac(n)
    if n == 1 then
        return 1
    else 
        return n * fac(n - 1)
    end;
end

print(fac(10))
print(math.sin(8) + math.cos(10))


squrars = {1, 2, 3, 4,5}

for i, v in ipairs(squrars) do
    print(string.format('%d = %d', i, v))
end


Set = {}

function Set.new(t)
    local set = {}
    for _, l in ipairs(t) do set[l] = true end
    return set
end

function Set.intersection(a, b)
    local res = Set.new()
    for _, k in ipairs(a) do 
        res[k] = b[k]
    end
    return res
end

function Set.tostring(set)
    local s = "("
    local sep = ""
    for e in pairs(set) do 
        s = s .. sep .. e
        sep = ", "
    end
    return s .. "}"
end

a = {1, 2,3 ,4 , 5, 6, 7, 8, 9}
b = {2342, 2342, 2342313, 2432343 , 343}

print('enter a number:')
a = io.read('*number')
print(fac(a))


