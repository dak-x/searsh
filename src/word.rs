use std::collections::BTreeSet;

pub(crate) type Word = String;
struct WordIndex {
    word: String,
    docs: BTreeSet<usize>,
}

impl WordIndex {
    fn new(word: String) -> WordIndex {
        WordIndex {
            word,
            docs: docs.collect(),
        }
    }
}
