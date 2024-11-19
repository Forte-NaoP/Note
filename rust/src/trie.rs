fn encode(s: &str) -> i64 {
    let mut ret = 0;
    for c in s.chars() {
        ret <<= 5;
        ret |= c as i64 - 'A' as i64 + 1;
    }
    ret
}

fn decode(mut n: i64) -> String {
    let mut ret: Vec<char> = vec![];
    while n > 0 {
        ret.push(((n & 31) as u8 + 'A' as u8 - 1) as char);
        n >>= 5;
    }
    ret.reverse();
    ret.iter().collect()
}

struct Trie {
    next: [Option<Box<Trie>>; 26],
    end: bool,
}

impl Trie {
    fn new() -> Self {
        Trie {
            next: Default::default(),
            end: false,
        }
    }

    fn insert(&mut self, s: i64) {
        if s == 0 {
            self.end = true;
            return
        }
        let c = (s & 31) as usize - 1;
        if self.next[c].is_none() {
            self.next[c] = Some(Box::new(Trie::new()));
        }
        self.next[c].as_mut().unwrap().insert(s >> 5);
    }

    fn find(&self, s: i64) -> bool {
        if s == 0 {
            return self.end
        }
        let c = (s & 31) as usize - 1;
        if self.next[c].is_none() {
            return false
        }
        self.next[c].as_ref().unwrap().find(s >> 5)
    }
}