use tini::Ini;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Score {
    pub name: String,
    pub score: u32,
    pub time: String,
    pub last: bool,
}

#[derive(Debug)]
pub struct ScoreTable {
    users: Vec<Score>,
}

impl Score {
    fn new(name: String, score: u32, time: String, last: bool) -> Score {
        Score { name, score, time, last }
    }
}

impl ScoreTable {
    pub fn from_config(config: &Ini) -> ScoreTable {
        let user: Vec<String> = config.get_vec("score", "users").unwrap_or_default();
        let score: Vec<u32> = config.get_vec("score", "scores").unwrap_or_default();
        let time: Vec<String> = config.get_vec("score", "times").unwrap_or_default();
        let mut users = Vec::new();
        for (u, s, t) in user.into_iter().zip(score).zip(time).map(|((x, y), z)| (x, y, z)) {
            users.push(Score::new(u, s, t, false));
        }
        let mut game_table = ScoreTable { users };
        game_table.sort_by_score();
        game_table
    }

    pub fn get_highscore(&self) -> u32 {
        if self.users.is_empty() {
            0
        } else {
            self.users[0].score
        }
    }

    pub fn update_config(mut self, count: usize, config: Ini) -> Ini {
        self.sort_by_score();
        let mut users = Vec::new();
        let mut scores = Vec::new();
        let mut times = Vec::new();
        for Score { name, score, time, .. } in self.users.into_iter().take(count) {
            users.push(name);
            scores.push(format!("{}", score));
            times.push(time);
        }
        config
            .section("score")
            .item("users", users.as_slice().join(","))
            .item("scores", scores.as_slice().join(","))
            .item("times", times.as_slice().join(","))
    }

    pub fn push(&mut self, name: String, score: u32, time: String) {
        for item in self.users.iter_mut() {
            item.last = false;
        }
        self.users.push(Score::new(name, score, time, true));
        self.sort_by_score();
    }

    pub fn iter(&self) -> std::slice::Iter<Score> {
        self.users.iter()
    }

    fn sort_by_score(&mut self) {
        self.users.sort_by(|b, a| a.score.cmp(&b.score));
    }
}
