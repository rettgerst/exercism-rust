#[derive(Debug)]
pub struct HighScores {
    scorevec: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scorevec: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scorevec[..]
    }

    pub fn latest(&self) -> Option<u32> {
        self.scorevec.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scorevec.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted = self.scorevec.clone();
        sorted.sort();

        sorted.into_iter().rev().take(3).collect()
    }
}
