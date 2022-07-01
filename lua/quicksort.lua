function qsort(arr, l, r)
    if l >= r then
        return
    end
    local mid = math.floor((l + r )/2)
    local x = arr[mid]
    local i=l
    local j=r
    repeat
        while arr[i] < x do i = i + 1 end
        while arr[j] > x do j = j - 1 end
        if i <= j then
            arr[i], arr[j] = arr[j], arr[i]
            i = i + 1 
            j = j - 1
        end
    until i<=j
    if l < j then qsort(arr, l, j) end
    if i < r then qsort(arr, i, r) end
end

function getArrSize(arr)
    siz = 0
    for i in pairs(arr) do 
        siz = siz + 1
    end
    return siz
end

arr = {2342, 22, 3421, 234256, 123, 23245, 884, 8, 23, 998, 98983, 112, 34,5,33}
arrlen = getArrSize(arr)

qsort(arr,1, arrlen)

for i in pairs(arr) do
    print(arr[i])
end

