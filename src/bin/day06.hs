module Main where
import qualified Data.Set as S

solve :: Int -> Int -> String -> Int
solve _ _ [] = 0
solve win n xs
    | S.size (S.fromList $ take win xs) == win = win + n
    | otherwise = solve win (n + 1) (tail xs)

main :: IO ()
main = do
    input <- readFile "../../input/input06.txt"
    print $ solve 4 0 input
    print $ solve 14 0 input
