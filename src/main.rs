use std::io;
use permutation_enum::output;

fn main() -> io::Result<()> {
    let n: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(3);
    output::stream_all(n)
}
