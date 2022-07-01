local Vector = {}

function Vector.new(x, y)
    local self = {x=x, y=y}

    function self.len()
        return math.sqrt(self.x ^ 2 + self.y ^ 2 )
    end

    function self.add(otherVec)
        return Vector.new(self.x + otherVec.x, self.y + otherVec.y)
    end

    function self.sub(otherVec)
        return Vector.new(self.x - otherVec.x , self.y + otherVec.y)
    end

    function self.dot(otherVec)
        return self.x * otherVec.x + self.y * otherVec.y
    end

    function self.cross(otherVec)
        return self.x * otherVec.y  - self.y * otherVec.x  
    end

    function self.distance(otherVec)
        return math.sqrt((self.x - otherVec.x)^2 + (self.y - otherVec.y)^2)
    end

    function self.printf()
        print(string.format('(%d, %d)', self.x, self.y))
    end

    return self
end


local x = Vector.new(1, 1)
local y = Vector.new(4, 2)

print( x.distance(y) )

