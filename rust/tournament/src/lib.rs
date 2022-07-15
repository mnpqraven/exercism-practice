use std::str::FromStr;

/// "Given the result of the played matches '{}' return a properly formatted tally table string.",
/// * `match_results`:
// TODO: struct
pub struct Team {
    name: String,
    wins: u8,
    losses: u8,
    draw: u8
}
pub fn tally(match_results: &str) -> String {
    // TODO: board builder
    // TODO: points calculator
    let split = match_results.split(";");
    let first = split.nth(0);
    let second = split.nth(1);
    let outcome = split.nth(2);

}
