use std::ops::{BitAnd, BitOr, BitXor, Deref};

#[derive(Hash, Copy, Clone, PartialEq, Eq, Debug)]
pub struct BinSeq(pub [bool; 7]);

impl BitXor for BinSeq {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        BinSeq(self.0.zip(rhs.0).map(|(l, r)| l.bitxor(r)))
    }
}

impl BitAnd for BinSeq {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        BinSeq(self.0.zip(rhs.0).map(|(l, r)| l.bitand(r)))
    }
}

impl BitOr for BinSeq {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        BinSeq(self.0.zip(rhs.0).map(|(l, r)| l.bitor(r)))
    }
}

impl PartialOrd for BinSeq {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let implies = |(a, b): &(bool, bool)| *b || !*a;
        let included_in_other = self.0.zip(other.0).iter().all(implies);
        let including_oter = other.0.zip(other.0).iter().all(implies);
        match (included_in_other, including_oter) {
            (false, false) => None,
            (true, false) => Some(std::cmp::Ordering::Less),
            (false, true) => Some(std::cmp::Ordering::Greater),
            (true, true) => Some(std::cmp::Ordering::Equal),
        }
    }
}

impl BinSeq {
    pub fn card(&self) -> usize {
        self.0.into_iter().filter(|b| *b).count()
    }
}

// impl Into<HashSet<usize>> for BinSeq {
//     fn into(self) -> HashSet<usize> {
//         self.0
//             .iter()
//             .enumerate()
//             .filter_map(|(n, bit)| bit.then_some(n))
//             .collect()
//     }
// }
