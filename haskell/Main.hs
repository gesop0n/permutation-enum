-- {1..n} の全順列を辞書順に列挙して印字する。
--
-- 「残りから1つ選び、残りを再帰する」という定義そのもの（選択ベース再帰）。
-- 出力フォーマットは Rust 版（rust/src/output.rs）と同じ「空白区切り・1行1順列」。
--
-- 計算量（出力に関する演算を除く）:
--   * メモリ : 遅延リストを印字しながら捨てるので、生きるのは「現在の再帰パス」だけ。
--              n に対して多項式（n! には依存しない）。
--              ※ 結果リスト全体を保持すると O(n!) に膨れる。`mapM_` で流す前提。
--   * 演算   : delete は O(長さ) だが、高コストなノードは浅い側（個数が少ない）に
--              集中するため総和が収束し Θ(n!)。各順列を長さ n の不変リストとして
--              組み立てる cons（Θ(n·n!)）は「出力の材料化」として除外。
module Main (main) where

import Data.List (delete)
import System.Environment (getArgs)

-- xs の全順列を辞書順に返す。xs が昇順なら出力も辞書順。
perms :: Eq a => [a] -> [[a]]
perms [] = [[]]
perms xs = [x : p | x <- xs, p <- perms (delete x xs)]

main :: IO ()
main = do
  args <- getArgs
  let n = case args of
        (a : _) -> read a -- 第1引数を n として読む
        [] -> 3 -- 既定値は Rust 版に合わせて 3
  mapM_ (putStrLn . unwords . map show) (perms [1 .. n :: Int])
