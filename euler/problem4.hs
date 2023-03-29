-- https://projecteuler.net/problem=4

palindromeProduct :: Int -> [Int]
palindromeProduct limit = foldr (\x r -> (foldr (\y r2 -> if (read $ reverse $ show (x*y)) == x*y then (x*y) : r2 else r2) [] [100..limit]) ++ r) [] [100..limit]

higherPalindrome = maximum $ palindromeProduct 1000
