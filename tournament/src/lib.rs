use std::collections::HashMap;

#[derive(Default)]
struct TeamStats {
    wins: u8,
    draws: u8,
    losses: u8,
}

impl TeamStats {
    fn points(&self) -> u8 {
        (3 * self.wins) + self.draws
    }

    fn update(&mut self, result: Result) {
        use Result::*;
        match result {
            Win => self.wins += 1,
            Loss => self.losses += 1,
            Draw => self.draws += 1,
        }
    }

    fn total(&self) -> u8 {
        self.wins + self.draws + self.losses
    }
}

#[derive(Copy, Clone)]
enum Result {
    Win,
    Loss,
    Draw,
}

impl Result {
    fn invert(&self) -> Result {
        use Result::*;
        match *self {
            Win => Loss,
            Loss => Win,
            other => other,
        }
    }
}

fn parse_line(line: &str) -> Option<(String, String, Result)> {
    if line.contains('\n') {
        return None;
    }
    let items: Vec<_> = line.split(";").collect();
    if items.len() != 3 {
        return None;
    }
    use Result::*;
    let result = match items[2] {
        "win" => Win,
        "loss" => Loss,
        "draw" => Draw,
        _ => return None,
    };
    return Some((items[0].to_string(), items[1].to_string(), result));
}

pub fn tally(input: &str) -> String {
    let mut statistics = HashMap::new();

    for line in input.split("\n") {
        if line.len() > 0 {
            if let Some((team1, team2, result)) = parse_line(line) {
                statistics.entry(team1).or_insert(TeamStats::default()).update(result);
                statistics.entry(team2).or_insert(TeamStats::default()).update(result.invert());
            }
        }
    }

    let mut teams: Vec<_> = statistics.keys().collect();
    teams.sort_by_key(|&k| statistics.get(k).unwrap().points());
    teams.reverse();

    let mut output = "Team                           | MP |  W |  D |  L |  P\n".to_string();
    for team in teams {
        let stats = statistics.get(team).unwrap();
        output = output +
                 &format!("{name:30} | {mp:>2} | {w:>2} | {d:>2} | {l:>2} | {p:>2}\n",
                          name = team,
                          mp = stats.total(),
                          w = stats.wins,
                          d = stats.draws,
                          l = stats.losses,
                          p = stats.points())
    }
    output.trim().to_string()
}
