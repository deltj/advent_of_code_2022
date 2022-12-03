use std::io::BufRead;

/// The set of possible player moves in a Rock-Paper-Scissors game
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

/// The set of possible outcomes for a round in a Rock-Paper-Scissors game
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Outcome {
    Lose,
    Draw,
    Win,
}

/// A structure representing a Rock-Paper-Scissors strategy where the moves of both
/// players are known, and the outcome is to be determined.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Strat1 {
    opp_move: RPS,
    own_move: RPS,
}

/// A structure representing a Rock-Paper-Scissors strategy where the opponent move
/// and round outcome are known, and the own-player move is to be determined.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Strat2 {
    opp_move: RPS,
    outcome: Outcome,
}

/// A fully-defined RPS round
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPSRound {
    opp_move: RPS,
    own_move: RPS,
    outcome: Outcome,
}


/// Given a string containing an input character in the set [A, B, C, X, Y, Z] return
/// the corresponding RPS move as defined in the AOC 2022 day 1 description
fn get_rps(symbol: &str) -> Option<RPS> {
    match symbol {
        "A" => Some(RPS::Rock),
        "B" => Some(RPS::Paper),
        "C" => Some(RPS::Scissors),
        "X" => Some(RPS::Rock),
        "Y" => Some(RPS::Paper),
        "Z" => Some(RPS::Scissors),
        &_ => None,
    }
}

/// Given a string containing an input character in the set [X, Y ,Z] return the
/// corresponding RPS outcome as defined in the AOC 2022 day 2 description
fn get_outcome(symbol: &str) -> Option<Outcome> {
    match symbol {
        "X" => Some(Outcome::Lose),
        "Y" => Some(Outcome::Draw),
        "Z" => Some(Outcome::Win),
        &_ => None,
    }
}

/// Read type-1 strategies from the provided buffer
pub fn read_strategy1_vector(reader: &mut dyn BufRead) -> Vec<Strat1> {
    let mut strat_vector: Vec<Strat1> = Vec::new();

    for line_result in reader.lines() {

        let line = line_result.unwrap();
        let tokens = line.trim().split(" ").collect::<Vec<&str>>();
        let strat: Strat1 = Strat1 {
            opp_move: get_rps(tokens[0]).unwrap(),
            own_move: get_rps(tokens[1]).unwrap(),
        };
        strat_vector.push(strat);
    }

    return strat_vector;
}

/// Read type-2 strategies from the provided buffer
pub fn read_strategy2_vector(reader: &mut dyn BufRead) -> Vec<Strat2> {
    let mut strat_vector: Vec<Strat2> = Vec::new();

    for line_result in reader.lines() {

        let line = line_result.unwrap();
        let tokens = line.trim().split(" ").collect::<Vec<&str>>();
        let strat: Strat2 = Strat2 {
            opp_move: get_rps(tokens[0]).unwrap(),
            outcome: get_outcome(tokens[1]).unwrap(),
        };
        strat_vector.push(strat);
    }

    return strat_vector;
}

/// Given a type-1 strategy, determine the round outcome
fn round_outcome(strat: &Strat1) -> Outcome {
    match strat {
        Strat1 { own_move: RPS::Rock, opp_move: RPS::Rock} => Outcome::Draw,
        Strat1 { own_move: RPS::Rock, opp_move: RPS::Paper} => Outcome::Lose,
        Strat1 { own_move: RPS::Rock, opp_move: RPS::Scissors} => Outcome::Win,
        Strat1 { own_move: RPS::Paper, opp_move: RPS::Rock} => Outcome::Win,
        Strat1 { own_move: RPS::Paper, opp_move: RPS::Paper} => Outcome::Draw,
        Strat1 { own_move: RPS::Paper, opp_move: RPS::Scissors} => Outcome::Lose,
        Strat1 { own_move: RPS::Scissors, opp_move: RPS::Rock} => Outcome::Lose,
        Strat1 { own_move: RPS::Scissors, opp_move: RPS::Paper} => Outcome::Win,
        Strat1 { own_move: RPS::Scissors, opp_move: RPS::Scissors} => Outcome::Draw
    }
}

/// Given a type-2 strategy, determine the own-player's move
fn round_move(strat: &Strat2) -> RPS {
    match strat {
        Strat2 { opp_move: RPS::Rock, outcome: Outcome::Lose} => RPS::Scissors,
        Strat2 { opp_move: RPS::Rock, outcome: Outcome::Draw} => RPS::Rock,
        Strat2 { opp_move: RPS::Rock, outcome: Outcome::Win} => RPS::Paper,
        Strat2 { opp_move: RPS::Paper, outcome: Outcome::Lose} => RPS::Rock,
        Strat2 { opp_move: RPS::Paper, outcome: Outcome::Draw} => RPS::Paper,
        Strat2 { opp_move: RPS::Paper, outcome: Outcome::Win} => RPS::Scissors,
        Strat2 { opp_move: RPS::Scissors, outcome: Outcome::Lose} => RPS::Paper,
        Strat2 { opp_move: RPS::Scissors, outcome: Outcome::Draw} => RPS::Scissors,
        Strat2 { opp_move: RPS::Scissors, outcome: Outcome::Win} => RPS::Rock,
    }
}

/// Solve each round the RPS game given a vector of type-1 strategies
pub fn solve_type1(strategy_vector: &Vec<Strat1>) -> Vec<RPSRound> {
    let mut round_vector: Vec<RPSRound> = Vec::new();
    for strat in strategy_vector {
        let outcome = round_outcome(strat);
        let round: RPSRound = RPSRound {
            opp_move: strat.opp_move,
            own_move: strat.own_move,
            outcome:  outcome,
        };
        round_vector.push(round);
    }
    return round_vector;
}

/// Solve each round in the RPS game given a vector of type-2 strategies
pub fn solve_type2(strategy_vector: &Vec<Strat2>) -> Vec<RPSRound> {
    let mut round_vector: Vec<RPSRound> = Vec::new();
    for strat in strategy_vector {
        let own_move = round_move(strat);
        let round: RPSRound = RPSRound {
            opp_move: strat.opp_move,
            own_move: own_move,
            outcome:  strat.outcome,
        };
        round_vector.push(round);
    }
    return round_vector;
}

/// Return the score associated with the provided RPS move
fn rps_score(rps: &RPS) -> u32 {
    match rps {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    }
}

/// Return the score associated with the provided RPS outcome
fn outcome_score(outcome: &Outcome) -> u32 {
    match outcome {
        Outcome::Lose => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    }
}

/// Determine the score of an RPS round
fn round_score(round: &RPSRound) -> u32 {
    rps_score(&round.own_move) + outcome_score(&round.outcome)
}

/// Determine the score of an RPS game (a vector of rounds)
pub fn total_score(round_vector: &Vec<RPSRound>) -> u32 {
    let mut total: u32 = 0;
    for round in round_vector {
        total += round_score(round);
    }
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_outcome_test() {
        assert_eq!(Outcome::Draw, round_outcome(&Strat1 { own_move: RPS::Rock, opp_move: RPS::Rock}));
        assert_eq!(Outcome::Lose, round_outcome(&Strat1 { own_move: RPS::Rock, opp_move: RPS::Paper}));
        assert_eq!(Outcome::Win,  round_outcome(&Strat1 { own_move: RPS::Rock, opp_move: RPS::Scissors}));
        assert_eq!(Outcome::Win,  round_outcome(&Strat1 { own_move: RPS::Paper, opp_move: RPS::Rock}));
        assert_eq!(Outcome::Draw, round_outcome(&Strat1 { own_move: RPS::Paper, opp_move: RPS::Paper}));
        assert_eq!(Outcome::Lose, round_outcome(&Strat1 { own_move: RPS::Paper, opp_move: RPS::Scissors}));
        assert_eq!(Outcome::Lose, round_outcome(&Strat1 { own_move: RPS::Scissors, opp_move: RPS::Rock}));
        assert_eq!(Outcome::Win,  round_outcome(&Strat1 { own_move: RPS::Scissors, opp_move: RPS::Paper}));
        assert_eq!(Outcome::Draw, round_outcome(&Strat1 { own_move: RPS::Scissors, opp_move: RPS::Scissors}));
    }

    #[test]
    fn round_move_test() {
        assert_eq!(RPS::Scissors, round_move(&Strat2 { opp_move: RPS::Rock, outcome: Outcome::Lose}));
        assert_eq!(RPS::Rock,     round_move(&Strat2 { opp_move: RPS::Rock, outcome: Outcome::Draw}));
        assert_eq!(RPS::Paper,    round_move(&Strat2 { opp_move: RPS::Rock, outcome: Outcome::Win}));
        assert_eq!(RPS::Rock,     round_move(&Strat2 { opp_move: RPS::Paper, outcome: Outcome::Lose})); 
        assert_eq!(RPS::Paper,    round_move(&Strat2 { opp_move: RPS::Paper, outcome: Outcome::Draw}));
        assert_eq!(RPS::Scissors, round_move(&Strat2 { opp_move: RPS::Paper, outcome: Outcome::Win}));
        assert_eq!(RPS::Paper,    round_move(&Strat2 { opp_move: RPS::Scissors, outcome: Outcome::Lose}));
        assert_eq!(RPS::Scissors, round_move(&Strat2 { opp_move: RPS::Scissors, outcome: Outcome::Draw}));
        assert_eq!(RPS::Rock,     round_move(&Strat2 { opp_move: RPS::Scissors, outcome: Outcome::Win}));
    }

    #[test]
    fn rps_score_test() {
        assert_eq!(1, rps_score(&RPS::Rock));
        assert_eq!(2, rps_score(&RPS::Paper));
        assert_eq!(3, rps_score(&RPS::Scissors));
    }

    #[test]
    fn round_score_test() {
        assert_eq!(8, round_score(&RPSRound { opp_move: RPS::Rock, own_move: RPS::Paper, outcome: Outcome::Win}));
        assert_eq!(1, round_score(&RPSRound { opp_move: RPS::Paper, own_move: RPS::Rock, outcome: Outcome::Lose}));
        assert_eq!(6, round_score(&RPSRound { opp_move: RPS::Scissors, own_move: RPS::Scissors, outcome: Outcome::Draw}));
    }

    #[test]
    fn one_line_part1() {
        let input =
            "A Y";
        let mut buf = input.as_bytes();
        let strategy_vector = read_strategy1_vector(&mut buf);
        assert_eq!(1, strategy_vector.len());
        assert_eq!(RPS::Rock, strategy_vector[0].opp_move);
        assert_eq!(RPS::Paper, strategy_vector[0].own_move);

        let round_vector = solve_type1(&strategy_vector);
        assert_eq!(1, round_vector.len());
        assert_eq!(RPSRound { opp_move: RPS::Rock, own_move: RPS::Paper, outcome: Outcome::Win}, round_vector[0]);

        let score = total_score(&round_vector);
        assert_eq!(8, score);
    }

    #[test]
    fn one_line_part2() {
        let input =
            "A Y";
        let mut buf = input.as_bytes();
        let strategy_vector = read_strategy2_vector(&mut buf);
        assert_eq!(1, strategy_vector.len());
        assert_eq!(RPS::Rock, strategy_vector[0].opp_move);
        assert_eq!(Outcome::Draw, strategy_vector[0].outcome);

        let round_vector = solve_type2(&strategy_vector);
        assert_eq!(1, round_vector.len());
        assert_eq!(RPSRound { opp_move: RPS::Rock, own_move: RPS::Rock, outcome: Outcome::Draw}, round_vector[0]);

        let score = total_score(&round_vector);
        assert_eq!(4, score);
    }
}