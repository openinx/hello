
local Trie = {}
local alpha = '1234567890abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ'

function newNode(size)
    local node = {count=0,arr={}}
    for i=1,size do 
        node.arr[i] = nil
    end
    return node
end

function index(s, i)
    return string.find(alpha, string.sub(s, i, i))
end

function Trie.new(n)
    local self = {size=n,root=newNode(n)}

    function self.insert(str)
        local c = self.root
        for i=1,string.len(str) do
            local idx = index(str, i)
            local x = c.arr[idx]
            if x == nil then
                local node = newNode(self.size)
                c.arr[idx] = node
            end
            c = c.arr[idx]
        end
        c.count = c.count + 1
    end

    function self.find(str)
        local c = self.root
        for i=1,string.len(str) do
            local idx = index(str, i)
            local x = c.arr[idx]
            if x == nil then 
                return 0
            end
            c = c.arr[idx]
        end
        if not (c == nil) and not (c.count == 0) then
            return c.count
        else 
            return 0
        end
    end

    return self
end

math.randomseed(os.time())
function getRandomStr(len)
    local x = {}
    for i=1,len do
        local k = math.random(1, string.len(alpha))
        table.insert(x, string.sub(alpha, k, k))
    end
    return table.concat(x)
end


function unitTest(n)
    local all={} x={}
    for i=1,n do 
        local s = getRandomStr(3)
        table.insert(all, s)
    end
    for _, v in ipairs(all) do 
        if x[v] == nil then
            x[v] = 1
        else
            x[v] = x[v] + 1
        end
    end

    local trie = Trie.new(10)
    for _, v in ipairs(all) do
        trie.insert(v)
    end

    -- for _, v in ipairs(all) do
    --    print('table[' .. v .. '] = ' .. x[v])
    -- end

    for _, v in ipairs(all) do
        hc = x[v]
        tc = trie.find(v)
        msg = 'string: ' .. v ..' hashTable.Count: ' .. hc .. ' Trie.Count: ' .. tc .. ' is not equal.'
        assert(hc == tc, msg)
    end

end

unitTest(1000000)

