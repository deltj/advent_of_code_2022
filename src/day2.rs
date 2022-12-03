use std::io::BufRead;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum RoundOutcome {
    Lose,
    Draw,
    Win,
}

pub struct Strat {
    opp_move: RPS,
    own_move: RPS,
}

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

fn round_outcome(strat: &Strat) -> RoundOutcome {
    match strat {
        Strat { own_move: RPS::Rock, opp_move: RPS::Rock} => RoundOutcome::Draw,
        Strat { own_move: RPS::Rock, opp_move: RPS::Paper} => RoundOutcome::Lose,
        Strat { own_move: RPS::Rock, opp_move: RPS::Scissors} => RoundOutcome::Win,
        Strat { own_move: RPS::Paper, opp_move: RPS::Rock} => RoundOutcome::Win,
        Strat { own_move: RPS::Paper, opp_move: RPS::Paper} => RoundOutcome::Draw,
        Strat { own_move: RPS::Paper, opp_move: RPS::Scissors} => RoundOutcome::Lose,
        Strat { own_move: RPS::Scissors, opp_move: RPS::Rock} => RoundOutcome::Lose,
        Strat { own_move: RPS::Scissors, opp_move: RPS::Paper} => RoundOutcome::Win,
        Strat { own_move: RPS::Scissors, opp_move: RPS::Scissors} => RoundOutcome::Draw
    }
}

fn rps_score(rps: &RPS) -> u32 {
    match rps {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    }
}

fn strat_score(strat: &Strat) -> u32 {
    let base_score = rps_score(&strat.own_move);
    match round_outcome(strat) {
        RoundOutcome::Lose => base_score,
        RoundOutcome::Draw => base_score + 3,
        RoundOutcome::Win  => base_score + 6,
    }
}

pub fn total_score(strat_vector: &Vec<Strat>) -> u32 {
    let mut total: u32 = 0;
    for strat in strat_vector {
        total += strat_score(strat);
    }
    return total;
}

pub fn read_strategy_vector(reader: &mut dyn BufRead) -> Vec<Strat> {
    let mut strat_vector: Vec<Strat> = Vec::new();

    for line_result in reader.lines() {

        let line = line_result.unwrap();
        let tokens = line.trim().split(" ").collect::<Vec<&str>>();
        let strat: Strat = Strat {
            opp_move: get_rps(tokens[0]).unwrap(),
            own_move: get_rps(tokens[1]).unwrap(),
        };
        strat_vector.push(strat);
    }

    return strat_vector;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_outcome_test() {
        assert_eq!(RoundOutcome::Draw, round_outcome(&Strat { own_move: RPS::Rock, opp_move: RPS::Rock}));
        assert_eq!(RoundOutcome::Lose, round_outcome(&Strat { own_move: RPS::Rock, opp_move: RPS::Paper}));
        assert_eq!(RoundOutcome::Win,  round_outcome(&Strat { own_move: RPS::Rock, opp_move: RPS::Scissors}));
        assert_eq!(RoundOutcome::Win,  round_outcome(&Strat { own_move: RPS::Paper, opp_move: RPS::Rock}));
        assert_eq!(RoundOutcome::Draw, round_outcome(&Strat { own_move: RPS::Paper, opp_move: RPS::Paper}));
        assert_eq!(RoundOutcome::Lose, round_outcome(&Strat { own_move: RPS::Paper, opp_move: RPS::Scissors}));
        assert_eq!(RoundOutcome::Lose, round_outcome(&Strat { own_move: RPS::Scissors, opp_move: RPS::Rock}));
        assert_eq!(RoundOutcome::Win,  round_outcome(&Strat { own_move: RPS::Scissors, opp_move: RPS::Paper}));
        assert_eq!(RoundOutcome::Draw, round_outcome(&Strat { own_move: RPS::Scissors, opp_move: RPS::Scissors}));
    }

    #[test]
    fn rps_score_test() {
        assert_eq!(1, rps_score(&RPS::Rock));
        assert_eq!(2, rps_score(&RPS::Paper));
        assert_eq!(3, rps_score(&RPS::Scissors));
    }

    #[test]
    fn strat_score_test() {
        assert_eq!(8, strat_score(&Strat { opp_move: RPS::Rock, own_move: RPS::Paper }));
        assert_eq!(1, strat_score(&Strat { opp_move: RPS::Paper, own_move: RPS::Rock }));
        assert_eq!(6, strat_score(&Strat { opp_move: RPS::Scissors, own_move: RPS::Scissors }));
    }

    #[test]
    fn one_line() {
        let input =
            "A Y";
        let mut buf = input.as_bytes();
        let strategy_vector = read_strategy_vector(&mut buf);
        assert_eq!(1, strategy_vector.len());

        assert_eq!(RPS::Rock, strategy_vector[0].opp_move);
        assert_eq!(RPS::Paper, strategy_vector[0].own_move);
    }
}