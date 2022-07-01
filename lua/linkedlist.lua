LinkedList = {}

function LinkedList.new()
    local initNode = {next=initNode,prev=initNode, value=0}
    local self={head=initNode,tail=initNode,size=0}

    function self.pushBack(value)
        if self.size == 0 then
            self.head.value = value
        else
            node = {next=self.head,prev=self.tail,value=value}
            self.head.prev=node
            self.tail.next=node
            self.tail = node
        end
        self.size = self.size + 1
    end

    function self.printf()
        print(string.format('list.printf() size=%d', self.size))
        local index = 0
        local iter = self.head
        while index <  self.size do
            index = index + 1
            print(string.format('#value: %d', iter.value))
            iter = iter.next
        end
    end 

    function self.pushFront(value)
        if self.size == 0 then
            self.head.value = value
        else
            node = {next=self.head,prev=self.tail,value=value}
            self.head.prev=node
            self.tail.prev=node
            self.head = node
        end
        self.size = self.size + 1
    end

    function self.remove(value)
        local index = 0
        local iter = self.head
        while index < self.size do 
            if iter.value == value then
                iter.prev.next = iter.next
                iter.next.prev = iter.prev
                if iter == self.head then self.head = self.head.next end
                if iter == self.tail then self.tail = self.tail.prev end
                self.size = self.size - 1
                break
            end
        end
    end

    return self
end


function main()
    list = LinkedList.new()
    for i=1,100 do 
        list.pushBack(i)
    end
    list.printf()

    for i=1,100 do 
        list.remove(i)
    end
    list.printf()
end

main()
