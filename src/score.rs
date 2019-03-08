use tini::Ini;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Score {
    pub name: String,
    pub score: u32,
    pub time: String
}

#[derive(Debug)]
pub struct ScoreTable {
    users: Vec<Score>
}

impl Score {
    fn new(name: String, score: u32, time: String) -> Score {
        Score { name, score, time }
    }
}

impl ScoreTable {
    pub fn from_config(config: &Ini) -> ScoreTable {
        let user: Vec<String> = config.get_vec("score", "users").unwrap_or(Vec::new());
        let score: Vec<u32> = config.get_vec("score", "scores").unwrap_or(Vec::new());
        let time: Vec<String> = config.get_vec("score", "times").unwrap_or(Vec::new());
        let mut users = Vec::new();
        for (u, s, t) in user.into_iter().zip(score).zip(time).map(|((x, y), z)| (x, y, z)) {
            users.push(Score::new(u, s, t));
        }
        let mut game_table = ScoreTable { users };
        game_table.sort_by_score();
        game_table
    }

    pub fn get_highscore(&self) -> u32 {
        if self.users.len() < 1 {
            0
        } else {
            self.users[0].score
        }
    }

    pub fn to_config(mut self, count: usize, config: Ini) -> Ini {
        self.sort_by_score();
        let mut users = Vec::new();
        let mut scores = Vec::new();
        let mut times = Vec::new();
        for Score { name, score, time } in self.users.into_iter().take(count) {
            users.push(name);
            scores.push(format!("{}", score));
            times.push(time);
        }
        config.section("score")
              .item("users", &users.as_slice().join(","))
              .item("scores", &scores.as_slice().join(","))
              .item("times", &times.as_slice().join(","))
    }

    pub fn push(&mut self, name: String, score: u32, time: String) {
        self.users.push(Score::new(name, score, time));
        self.sort_by_score();
    }

    pub fn iter(&self) -> std::slice::Iter<Score> {
        self.users.iter()
    }

    fn sort_by_score(&mut self) {
        self.users.sort_by(|b, a| a.score.cmp(&b.score));
    }
}