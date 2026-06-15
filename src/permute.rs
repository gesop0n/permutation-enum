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
