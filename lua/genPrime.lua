is = {}
primes = {}
maxn = 1000000

for i=2,maxn do 
    is[i] = true 
end

for i=2,maxn do 
    if is[i] then
        table.insert(primes, i)
        for j=i,maxn,i do
            is[j] = false
        end
    end
end

for i, v in ipairs(primes) do
    print(string.format('Prime[%d] = %d', i, v))
end

