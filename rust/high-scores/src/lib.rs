#[derive(Debug)]
pub struct HighScores<'a> {
    s: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { s: scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.s
    }

    pub fn latest(&self) -> Option<u32> {
        match self.s {
            &[] => None,
            _ => {
                let i = self.s.len() - 1;
                Some(self.s[i])
            }
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut v = Vec::from(self.s);
        if v.is_empty() {
            return None;
        }
        v.sort_by(|a, b| b.partial_cmp(a).unwrap());
        v.first().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut v = Vec::from(self.s);
        if v.is_empty() {
            return v;
        }
        v.sort_by(|a, b| b.partial_cmp(a).unwrap());
        v.into_iter().take(3).collect()
    }
}
