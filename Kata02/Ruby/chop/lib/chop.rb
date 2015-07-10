require "chop/version"

module Chop
    def Chop.chop(item, array)
        low = 0
        width = array.length
        while width > 0 do
            mid_index = width / 2 + low
            if array[mid_index] > item
            elsif array[mid_index] < item
                low = mid_index + 1
                width -= 1
            else
                return mid_index
            end
            width /= 2
        end
        -1
    end
end
