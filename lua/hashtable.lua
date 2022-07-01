local HashTable = {}

local alpha = '1234567890abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ'

function HashTable.new()
    local self = {gseq=0,cseq=0,cval=0,tab={}}

    function self.set(key, val)
        -- O(1)
        self.gseq = self.gseq + 1
        node = {seq=self.gseq, val=val}
        self.tab[key] = node
    end

    function self.get(key, val)
        -- O(1)
        local node = self.tab[key]
        if node == nil or node.seq < self.cseq then 
            return self.cval 
        end
        return node.val
    end

    function self.setall(val)
        -- O(1)
        self.gseq = self.gseq + 1
        self.cseq = self.gseq
        self.cval = val
    end
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


function unitTest(n,strlen)
    hashtable = HashTable.new()

    print('****')

    local cmds = {}
    local luatable = {}
    local op, key, val
    for i=1,n do 
        op = math.randomseed(1,3)
        if op == 1 then
            -- set key val
            key = getRandomStr(strlen)
            val = getRandomStr(strlen)
            table.insert(cmds,{op=op, key=key, val=val})
        elseif op == 2 then
            -- get key
            key = getRandomStr(strlen)
            table.insert(cmds,{op=op, key=key})
        elseif op == 3 then
            -- setall val
            val = getRandomStr(strlen)
            table.insert(cmds,{op=op, val=val})
        end
        -- print('.. i' .. i)
    end

    -- print(cmds)

    for k, v in ipairs(cmds) do
        print('** ' .. k .. ' ** ' .. v)
    end

    for i=1,#cmds do
        c = cmds[i]
        print('#####...' .. c.op)
        if c.op == 1 then
            -- set key val
            luatable[c.key] = c.val
        elseif c.op == 2 then
            -- get key
            luaval = luatable[c.key]
            hashval = hashtable.get(c.key)
            print('** luaval: ' .. luaval .. ' ** hashval ' .. hashval)
            assert(luaval == hashval,
                string.format('key: %s luaTable: %s hashtable: %s',
                c.key, luaval, hashval))
        elseif c.op == 3 then
            -- setall val
            hashtable.setall(c.val)
            for k,v in pairs(luatable) do
                luatable[k] = c.val
            end
        end
    end
end

unitTest(1000000, 4)
