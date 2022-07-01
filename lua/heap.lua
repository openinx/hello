local MaxHeap = {}

function MaxHeap.new()
    local self = {len=0, arr={}}

    function self.up(k)
        local i = k
        local j = math.floor(i/2)
        while i > 1 do
            j = math.floor(i/2)
            if self.arr[j] >= self.arr[i] then
                break
            else
                self.arr[j], self.arr[i] = self.arr[i], self.arr[j]
                i = j
            end
        end
    end

    function self.down()
        local i = 1 , j 
        -- print('self.len:' .. self.len)
        while  2*i <= self.len do
            -- self.printf()
            j = 2*i
            if j + 1 <= self.len and self.arr[j] < self.arr[j+1] then
                j = j + 1
            end
            if self.arr[j] < self.arr[i] then 
                break
            else
                self.arr[i], self.arr[j] = self.arr[j], self.arr[i]
                i = j
            end
        end
    end

    function self.push(x)
        table.insert(self.arr, x)
        self.len = self.len + 1
        self.up(self.len)
    end

    function self.pop()
        if self.len <= 0 then
            error('len <= 0')
        end
        local x = self.arr[1]
        self.arr[1] = self.arr[self.len]
        self.len = self.len - 1
        self.down()
        return x
    end

    function self.empty()
        return self.len == 0
    end

    function self.printf()
        s = 'len = ' .. self.len .. ' ['
        for i = 1,self.len do 
            s = s .. ' ' .. self.arr[i]
        end
        s = s .. ' ]'
        print(s)
    end

    return self
end


local heap = MaxHeap.new()

for i = 1,10000 do 
    heap.push(i)
    -- heap.printf()
end

while not heap.empty() do
   print(' *** ' .. heap.pop())
end

