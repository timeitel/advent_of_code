use serde::Deserialize;
use std::cmp::Ordering;

fn main() {
    let mut sum = 0;
    for (i, groups) in include_str!("input.txt").split("\n\n").enumerate() {
        let i = i + 1;

        let mut packets = groups
            .lines()
            .map(|line| serde_json::from_str::<Packet>(line).unwrap());
        let l = packets.next().unwrap();
        let r = packets.next().unwrap();

        if l < r {
            sum += i;
        }
    }
    dbg!(sum);
}

#[derive(Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum Packet {
    List(Vec<Packet>),
    Number(u32),
}

impl Packet {
    // TODO: not really understanding this
    fn with_slice<T>(&self, f: impl FnOnce(&[Packet]) -> T) -> T {
        match self {
            Self::List(n) => f(&n[..]),
            Self::Number(n) => f(&[Self::Number(*n)]),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Number(a), Packet::Number(b)) => a.partial_cmp(b),
            (l, r) => Some(l.with_slice(|l| {
                r.with_slice(|r| {
                    l.iter()
                        .zip(r.iter())
                        .map(|(aa, bb)| aa.cmp(bb))
                        .find(|&ord| ord != Ordering::Equal)
                        .unwrap_or_else(|| l.len().cmp(&r.len()))
                })
            })),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
