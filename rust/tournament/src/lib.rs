use std::collections::HashMap;

const HEADER: &'static str = "Team                           | MP |  W |  D |  L |  P";

struct Team {
    name: String,
    matches_played: i32,
    wins: i32,
    draws: i32,
    losses: i32,
    points: i32,
}
impl Team {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            matches_played: 0,
            wins: 0,
            draws: 0,
            losses: 0,
            points: 0,
        }
    }
    fn won(&mut self) {
        self.wins += 1;
        self.matches_played += 1;
        self.points += 3;
    }
    fn lost(&mut self) {
        self.losses += 1;
        self.matches_played += 1;
    }
    fn drew(&mut self) {
        self.draws += 1;
        self.matches_played += 1;
        self.points += 1;
    }
}

pub fn tally(match_results: &str) -> String {
    let mut team_table: HashMap<String, Team> = HashMap::new();

    for line in match_results.lines() {
        let mut fields = line.split(";");

        let first_team = fields.next().unwrap();
        let second_team = fields.next().unwrap();
        let result = fields.next().unwrap();

        update_score(&mut team_table, String::from(first_team), result, false);
        update_score(&mut team_table, String::from(second_team), result, true);
    }

    let mut list: Vec<&Team> = team_table.values().collect();
    list.sort_by(|left, right| {
        if left.points == right.points {
            left.name.partial_cmp(&right.name).unwrap()
        } else {
            right.points.partial_cmp(&left.points).unwrap()
        }
    });
    let mut output = String::from(HEADER);
    for team in list {
        output = output + "\n" + &team_to_table(team);
    }
    output
}

fn team_to_table(team: &Team) -> String {
    format!(
        "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
        team.name, team.matches_played, team.wins, team.draws, team.losses, team.points
    )
}

fn update_score(
    map: &mut HashMap<String, Team>,
    name: String,
    match_result: &str,
    on_receiving_end: bool,
) {
    let team = map.entry(name.clone()).or_insert(Team::new(&name));
    if !on_receiving_end {
        match match_result {
            "win" => team.won(),
            "loss" => team.lost(),
            "draw" => team.drew(),
            _ => (),
        }
    } else {
        match match_result {
            "win" => team.lost(),
            "loss" => team.won(),
            "draw" => team.drew(),
            _ => (),
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn bruh() {
        let input = "othi;brothi;win";
        let mut iter = input.split(";");
        assert_eq!(iter.next().unwrap(), "othi");
        assert_eq!(iter.next().unwrap(), "brothi");
        assert_eq!(iter.next().unwrap(), "win");
    }
}
