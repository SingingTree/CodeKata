import Data.Vector
import Test.HUnit

chopRecurse :: (Ord a) => Vector a -> a -> Int -> Int -> Int
chopRecurse xs item low width
  | width <= 0 = -1
  | xs ! index > item = chopRecurse xs item low ((width - 1) `quot` 2)
  | xs ! index < item = chopRecurse xs item (index + 1) (width `quot` 2)
  | otherwise = index
  where index = low + width `quot` 2

chop :: (Ord a) => Vector a -> a -> Int
chop xs item = chopRecurse xs item 0 (Data.Vector.length xs)

-- Tests
test1 = TestCase (do let vec = empty
                     assertEqual "chop 3 vec[]" (-1) (chop vec 3))
test2 = TestCase (do let vec = generate (1) (\i -> i * 2 + 1)
                     assertEqual "chop 3 vec[1]" (-1) (chop vec 3))
test3 = TestCase (do let vec = generate  (1) (\i -> i * 2 + 1)
                     assertEqual "chop 1 vec[1]" 0 (chop vec 1))
test4 = TestCase (do let vec = generate  (3) (\i -> i * 2 + 1)
                     assertEqual "chop 1 vec[1, 3, 5]" 0 (chop vec 1))
test5 = TestCase (do let vec = generate  (3) (\i -> i * 2 + 1)
                     assertEqual "chop 3 vec[1, 3, 5]" 1 (chop vec 3))
test6 = TestCase (do let vec = generate  (3) (\i -> i * 2 + 1)
                     assertEqual "chop 5 vec[1, 3, 5]" 2 (chop vec 5))

-- assert_equal(2,  chop(5, [1, 3, 5]))
-- assert_equal(-1, chop(0, [1, 3, 5]))
-- assert_equal(-1, chop(2, [1, 3, 5]))
-- assert_equal(-1, chop(4, [1, 3, 5]))
-- assert_equal(-1, chop(6, [1, 3, 5]))


tests = TestList [TestLabel "test1" test1,
                  TestLabel "test2" test2,
                  TestLabel "test3" test3,
                  TestLabel "test4" test4,
                  TestLabel "test5" test5,
                  TestLabel "test6" test6]

main = runTestTT tests
