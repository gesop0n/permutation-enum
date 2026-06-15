use std::io::{self, Write};

use crate::permute::Permutations;

fn push_usize(buf: &mut Vec<u8>, mut v: usize) {
    if v == 0 {
        buf.push(b'0');
        return;
    }
    let start = buf.len();
    while v > 0 {
        buf.push(b'0' + (v % 10) as u8);
        v /= 10;
    }
    buf[start..].reverse(); // 桁を逆に積んだので戻す
}

pub fn stream_all(n: usize) -> io::Result<()> {
    let mut perms = Permutations::new(n);
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let mut line: Vec<u8> = Vec::with_capacity(n * 4 + 1);

    while let Some(p) = perms.next_ref() {
        line.clear();
        for (k, &v) in p.iter().enumerate() {
            if k > 0 {
                line.push(b' ');
            }
            push_usize(&mut line, v);
        }
        line.push(b'\n');
        out.write_all(&line)?;
    }
    out.flush()
}
