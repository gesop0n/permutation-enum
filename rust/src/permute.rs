/// 次の辞書順列順列へ更新する
/// 既に最後（降順）なら、falseを返し、配列は変化しない
pub fn next_permutation<T: Ord>(a: &mut [T]) -> bool {
    if a.len() < 2 {
        return false;
    }

    // 1. a[i-1] < a[i] となる最大の`i`を探す
    let mut i = a.len() - 1;
    while i > 0 && a[i - 1] >= a[i] {
        i -= 1;
    }
    if i == 0 {
        return false; // 全体が降順 = 最後の順列
    }
    // 2. a[i-1]より大きい最右のjを探す
    let mut j = a.len() - 1;
    while a[j] <= a[i - 1] {
        j -= 1;
    }
    a.swap(i - 1, j);
    // 3. 接尾辞を反転（昇順に戻す）
    a[i..].reverse();
    true
}

/// {1, ..., n}の順列を辞書順に一個ずつ生成する。状態は配列１本のみ
pub struct Permutations {
    state: Vec<usize>,
    started: bool,
    done: bool,
}

impl Permutations {
    pub fn new(n: usize) -> Self {
        Self {
            state: (1..=n).collect(),
            started: false,
            done: false,
        }
    }

    /// アロケーションせずに次の順列をスライスで返す。
    pub fn next_ref(&mut self) -> Option<&[usize]> {
        if self.done {
            return None;
        }
        if !self.started {
            self.started = true; // 最初の１個（=ソート済み)をそのまま返す
            return Some(&self.state);
        }
        if next_permutation(&mut self.state) {
            Some(&self.state)
        } else {
            self.done = true;
            None
        }
    }
}

impl Iterator for Permutations {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_ref().map(<[usize]>::to_vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn factorial(n: usize) -> usize {
        (1..=n).product()
    }

    /// next_permutation を false になるまで回し、全列（初期値含む）を集める。
    fn enumerate_with_next_permutation(n: usize) -> Vec<Vec<usize>> {
        let mut a: Vec<usize> = (1..=n).collect();
        let mut out = vec![a.clone()];
        while next_permutation(&mut a) {
            out.push(a.clone());
        }
        out
    }

    // ---- next_permutation 単体 ----

    /// 1ステップで「次の辞書順」へ進み true を返す。
    #[test]
    fn next_permutation_steps_to_successor() {
        let mut a = vec![1, 2, 3];
        assert!(next_permutation(&mut a));
        assert_eq!(a, vec![1, 3, 2]);
    }

    /// 最後（降順）の順列では false を返し、配列は不変。
    #[test]
    fn next_permutation_at_last_returns_false_unchanged() {
        let mut a = vec![3, 2, 1];
        assert!(!next_permutation(&mut a));
        assert_eq!(a, vec![3, 2, 1]);
    }

    /// 長さ 0・1 は常に false、長さ 2 は 1 回だけ true。
    #[test]
    fn next_permutation_short_slices() {
        let mut empty: Vec<usize> = vec![];
        assert!(!next_permutation(&mut empty));

        let mut single = vec![7];
        assert!(!next_permutation(&mut single));
        assert_eq!(single, vec![7]);

        let mut pair = vec![1, 2];
        assert!(next_permutation(&mut pair));
        assert_eq!(pair, vec![2, 1]);
        assert!(!next_permutation(&mut pair));
    }

    /// 多重集合でも重複を作らず、相異なる順列だけを辞書順に生成する。
    #[test]
    fn next_permutation_handles_duplicates() {
        let mut a = vec![1, 1, 2];
        let mut seen = vec![a.clone()];
        while next_permutation(&mut a) {
            seen.push(a.clone());
        }
        assert_eq!(seen, vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]);
    }

    /// T: Ord であれば数値以外（char など）でも動く。
    #[test]
    fn next_permutation_is_generic_over_ord() {
        let mut s = vec!['a', 'b', 'c'];
        assert!(next_permutation(&mut s));
        assert_eq!(s, vec!['a', 'c', 'b']);
    }

    /// 全列が辞書順に厳密増加（重複なし）し、個数は n!、両端も一致する。
    #[test]
    fn next_permutation_enumerates_all_in_order() {
        for n in 0..=6 {
            let all = enumerate_with_next_permutation(n);
            assert_eq!(all.len(), factorial(n), "n={n} の個数");
            for w in all.windows(2) {
                assert!(w[0] < w[1], "n={n}: {:?} !< {:?}", w[0], w[1]);
            }
            if n > 0 {
                assert_eq!(all[0], (1..=n).collect::<Vec<_>>(), "n={n} 先頭");
                assert_eq!(
                    *all.last().unwrap(),
                    (1..=n).rev().collect::<Vec<_>>(),
                    "n={n} 末尾"
                );
            }
        }
    }

    // ---- Permutations（イテレータ） ----

    /// 列挙数はちょうど n!（n=0 は空順列 1 個）。
    #[test]
    fn permutations_count_is_factorial() {
        for n in 0..=7 {
            assert_eq!(Permutations::new(n).count(), factorial(n), "n={n}");
        }
    }

    /// イテレータの列は next_permutation を直接回した列と完全一致。
    #[test]
    fn permutations_match_next_permutation() {
        for n in 0..=6 {
            let via_iter: Vec<Vec<usize>> = Permutations::new(n).collect();
            assert_eq!(via_iter, enumerate_with_next_permutation(n), "n={n}");
        }
    }

    /// next_ref（借用版）と Iterator（Vec 版）が同じ列を返す。
    #[test]
    fn permutations_next_ref_matches_iterator() {
        let mut p = Permutations::new(4);
        let mut via_ref = Vec::new();
        while let Some(s) = p.next_ref() {
            via_ref.push(s.to_vec());
        }
        let via_iter: Vec<Vec<usize>> = Permutations::new(4).collect();
        assert_eq!(via_ref, via_iter);
    }

    /// 境界: n=0 は空順列 1 個、n=1 は [1] のみ。
    #[test]
    fn permutations_edge_cases() {
        assert_eq!(
            Permutations::new(0).collect::<Vec<_>>(),
            vec![Vec::<usize>::new()]
        );
        assert_eq!(Permutations::new(1).collect::<Vec<_>>(), vec![vec![1]]);
    }
}
