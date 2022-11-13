// Example
// const P1_START: usize = 4;
// const P2_START: usize = 8;

const P1_START: usize = 6;
const P2_START: usize = 10;

// Deterministic dice
#[derive(Debug)]
struct DDice {
    value: usize,
    side_count: usize,
    roll_count: usize,
}

impl DDice {
    fn new(sides: usize) -> Self {
        DDice {
            value: 0,
            side_count: sides,
            roll_count: 0,
        }
    }

    fn next(&mut self) -> usize {
        self.roll_count += 1;

        self.value = 1 + self.value % self.side_count;

        self.value
    }
}

#[derive(Debug, Copy, Clone)]
struct Player {
    position: usize,
    play_count: usize,
    score: usize,

    universe_count: usize,
}

impl Player {
    fn new(pos: usize) -> Self {
        Player {
            position: pos,
            play_count: 0,
            score: 0,
            universe_count: 1,
        }
    }

    fn play(&mut self, dice: &mut DDice) {
        let dice_points_sum =
            dice.next() + dice.next() + dice.next();

        self.position = 1 + ((self.position - 1) + dice_points_sum) % 10;

        self.score += self.position;
        self.play_count += 1;
    }

    fn play_with_sum(&mut self, dice_points_sum: usize, universe_count: usize) {
        self.position = 1 + ((self.position - 1) + dice_points_sum) % 10;

        self.score += self.position;
        self.play_count += 1;

        self.universe_count *= universe_count;
    }

    fn inc_universes(&mut self, v: usize) {
        self.universe_count *= v;
    }

    fn make_copy(&self) -> Self {
        Player {
            position: self.position,
            play_count: self.play_count,
            score: self.score,
            universe_count: self.universe_count,
        }
    }
}

fn part1() {
    let mut p1 = Player::new(P1_START);
    let mut p2 = Player::new(P2_START);

    let mut dice = DDice::new(100);


    loop {
        p1.play(&mut dice);

        if p1.score >= 1000 {
            break;
        }

        p2.play(&mut dice);

        if p2.score >= 1000 {
            break;
        }
    }

    println!("{:?}", p1);
    println!("{:?}", p2);
    println!("{:?}", dice);

    let lowest_score =
        if p1.score < p2.score {
            p1.score
        } else {
            p2.score
        };

    println!("\nresult: {}", dice.roll_count * lowest_score);
}

#[derive(Debug)]
struct UniverseScore {
    value: usize,
    count: usize,
}

// sum         | 3 | 4 | 5 | 6 | 7 | 8 | 9
// # universes | 1 | 3 | 6 | 7 | 6 | 3 | 1
const score_table: [UniverseScore; 7] = [
    UniverseScore { value: 3, count: 1 },
    UniverseScore { value: 4, count: 3 },
    UniverseScore { value: 5, count: 6 },
    UniverseScore { value: 6, count: 7 },
    UniverseScore { value: 7, count: 6 },
    UniverseScore { value: 8, count: 3 },
    UniverseScore { value: 9, count: 1 },
];

fn player_compare(p1: &Player, p2: &Player) -> bool {
   p1.position == p2.position && p1.score == p2.score && p1.play_count == p2.play_count
}

fn part2() {
    let mut players_universes: Vec<(Player, Player)> = vec![
        (Player::new(P1_START), Player::new(P2_START))
    ];

    // let mut players_universes_wins: Vec<(Player, Player)> = vec![];

    let mut player1_wins = 0;
    let mut player2_wins = 0;

    for k in 0..10 {
        let mut new_universes: Vec<(Player, Player)> = vec![];

        for p in &players_universes {
            for score in score_table {
                let mut new_player1 = p.0.make_copy();

                new_player1.play_with_sum(score.value, score.count);

                if new_player1.score >= 21 {
                    player1_wins += new_player1.universe_count;
                    continue;
                }

                let mut new_player2 = p.1.make_copy();
                new_player2.inc_universes(score.count);

                new_universes.push((new_player1, new_player2));
            }
        }

        players_universes = new_universes;

        println!("p1 plays ({})", k);
        println!("  p1 wins: {}", player1_wins);
        println!("  p2 wins: {}", player2_wins);

        if players_universes.len() == 0 {
            break;
        }

        new_universes = vec![];

        for p in &players_universes {
            for score in score_table {
                let mut new_player2 = p.1.make_copy();

                new_player2.play_with_sum(score.value, score.count);

                if new_player2.score >= 21 {
                    player2_wins += new_player2.universe_count;
                    continue;
                }

                let mut new_player1 = p.0.make_copy();
                new_player1.inc_universes(score.count);

                new_universes.push((new_player1, new_player2));
            }
        }

        players_universes = new_universes;

        println!("p2 plays ({})", k);
        println!("  p1 wins: {}", player1_wins);
        println!("  p2 wins: {}", player2_wins);

        if players_universes.len() == 0 {
            break;
        }
    }
}

fn main() {
    // part1();
    part2();
}
