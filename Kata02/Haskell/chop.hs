import Data.Vector
import Test.HUnit

chopRecurse :: (Ord a) => Vector a -> a -> Int -> Int -> Int
chopRecurse xs item low width
  | width <= 0 = -1
  | xs ! index > item = chopRecurse xs item low ((width - 1) `quot` 2)
  | xs ! index < item = chopRecurse xs item index ((width -1) `quot` 2)
  | otherwise = index
  where index = low + width `quot` 2

chop :: (Ord a) => Vector a -> a -> Int
chop xs item = chopRecurse xs item 0 (Data.Vector.length xs)

test1 = TestCase (do let vec = empty
                     assertEqual "chop 3 []" (-1) (chop vec 3))
tests = TestList [TestLabel "test1" test1]

main = runTestTT tests
