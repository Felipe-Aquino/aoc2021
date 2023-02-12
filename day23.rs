use std::collections::HashMap;

const STAYING_POSITIONS: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

fn energy_by_anphipod(anphipod: char) -> usize {
    match anphipod {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => unreachable!(),
    }
}

fn room_by_anphipod(anphipod: char) -> usize {
  match anphipod {
    'A' => 2,
    'B' => 4,
    'C' => 6,
    'D' => 8,
    _ => unreachable!(),
  }
}

fn anphipod_by_room(room: usize) -> char {
  match room {
    2 => 'A',
    4 => 'B',
    6 => 'C',
    8 => 'D',
    _ => unreachable!(),
  }
}

#[derive(Debug, Copy, Clone)]
struct Step {
  start: Position,
  end: Position,
  cost: usize,
}

impl Step {
    fn new(start: Position, end: Position, cost: usize) -> Self {
        Step {
            start,
            end,
            cost,
        }
    }
}

#[derive(Debug, Clone)]
struct Path {
  cost: usize,
  steps: Vec<Step>,
}

struct PathManager {
    current_path: Path,
  
    min_cost: usize,
    min_cost_path: Option<Path>,
    fields: Vec<[char; 11]>,

    tracked: HashMap<String, usize>,
    tracked_stack: Vec<Vec<Step>>,
    tracked_stack_costs: Vec<Option<usize>>,
}

impl PathManager {
    fn new(fields: Vec<[char; 11]>) -> Self {
        let path = Path {
            cost: 0,
            steps: vec![],
        };

        PathManager {
            current_path: path,
            min_cost: 1 << 32,
            min_cost_path: None,
            fields: fields,
            tracked: HashMap::new(),
            tracked_stack: vec![],
            tracked_stack_costs: vec![],
        }
    }

    fn fields_swap(&mut self, p1: &Position, p2: &Position) {
        let element = self.fields[p1.row][p1.col];
        self.fields[p1.row][p1.col] = self.fields[p2.row][p2.col];
        self.fields[p2.row][p2.col] = element;
    }

    fn fields_at(&self, p: &Position) -> char {
        self.fields[p.row][p.col]
    }

    fn fields_print(&self) {
        let strs: Vec<String> = self.fields
            .iter()
            .map(|line| line.iter().collect())
            .collect();

        for s in strs {
            println!("{}", s);
        }
        println!("");
    }

    fn fields_to_string(&self) -> String {
        self.fields
            .iter()
            .map(|line| line.iter().collect())
            .collect::<Vec<String>>()
            .join("")
    }

    fn follow_path(&mut self) {
        let mut total_cost = 0;

        self.fields_print();

        let steps = self.min_cost_path.as_ref().unwrap().steps.clone();

        let size = steps.len();
        let mut last_idx = 0;

        for i in 0..size {
            last_idx = i;
            let step = steps[i];

            let can_go_cost = self.anphipod_can_go_to(&step.start, &step.end);

            if let Some(cost) = can_go_cost {
                total_cost += cost;
                self.fields_swap(&step.start, &step.end);

                println!("cost: {} {}", cost, total_cost);
                self.fields_print();
            } else {
                println!("error when following step {:?} {:?}", step.start, step.end);
                break;
            }
        }

        for i in (0..=last_idx).rev() {
            let step = steps[i];
            self.fields_swap(&step.start, &step.end);
        }

        println!("total_cost: {}", total_cost);
    }

    fn anphipod_can_go_to(&self, start: &Position, end: &Position) -> Option<usize> {
        if start == end {
            println!("Warn(anphipod_can_go_to): start and end are equal {:?}", start);
            return None;
        }

        let space = self.fields_at(start);

        if space < 'A' || space > 'D' {
            println!("Warn(anphipod_can_go_to): invalid space {}", space);
            return None;
        }

        if self.fields_at(end) != '.' {
            return None;
        }

        // Testing if the anfipod can
        if end.row > 0 && (room_by_anphipod(space) != end.col) {
            return None;
        }

        if end.row == 0 && start.row == 0 {
            return None;
        }

        let mut steps = 0;

        // Testing if there's a anphipod blocking right above
        // ...
        //  A  <--
        //  B
        for row in 0..start.row {
            if self.fields[row][start.col] != '.' {
                return None;
            }

            steps += 1;
        }

        let mut start_col = start.col;
        let mut end_col = end.col;

        if start_col > end_col {
            let c = start_col;
            start_col = end_col;
            end_col = c;
        }

        for i in start_col..end_col {
            // Testing if there's a anphipod blocking in the hallway
            // ..C...
            // ^ ^ .
            //   | B
            //
            if i != start.col && self.fields[0][i] != '.' {
                return None;
            }

            steps += 1;
        }

        if end.row == 0 && (end.col % 2 == 0) && end.col != 0 && end.col != 10 {
            return None;
        }

        if end.row > 0 {
            let total_rows = self.fields.len();

            for row in 1..total_rows {
                let test_space = self.fields[row][end.col];

                if row <= end.row {
                    // Testing if the path is blocked
                    // ...
                    //  B  <--
                    //  .
                    if test_space != '.' {
                        return None;
                    }

                    steps += 1;
                } else {
                    // Testing if already exist a space
                    // ...
                    //  .
                    //  B  <--
                    if test_space != space {
                        return None;
                    }
                }
            }
        }

        Some(steps * energy_by_anphipod(space))
    }

    fn eval_step(&self) -> Vec<Step> {
        let mut new_steps = vec![];

        let total_rows = self.fields.len();

        for i in STAYING_POSITIONS {
            let space = self.fields[0][i];

            if space != '.' {
                let room = room_by_anphipod(space);

                let mut row_dest = self.fields.len() - 1;

                for j in (1..total_rows).rev() {
                    if self.fields[j][room] == '.' {
                        row_dest = j;
                        break;
                    }
                }

                let start = Position { row: 0, col: i };
                let end = Position { row: row_dest, col: room };

                let can_go_cost = self.anphipod_can_go_to(&start, &end);

                if let Some(cost) = can_go_cost {
                    let step = Step::new(start, end, cost);
                    new_steps.push(step);
                }
            }
        }

        for i in (2..10).step_by(2) {
            let mut init_row_opt = None;

            for j in 1..total_rows {
                if self.fields[j][i] != '.' {
                    init_row_opt = Some(j);
                    break;
                }
            }

            if let Some(init_row) = init_row_opt {
                let space = self.fields[init_row][i];
                let room = room_by_anphipod(space);

                // Testing if an anphipod can go to it's room
                if i != room {
                    let mut dest_row = total_rows - 1;

                    for j in (1..total_rows).rev() {
                        if self.fields[j][room] == '.' {
                            dest_row = j;
                            break;
                        }
                    }

                    let start = Position{ row: init_row, col: i };
                    let end = Position{ row: dest_row, col: room };

                    let can_go_cost = self.anphipod_can_go_to(&start, &end);

                    if let Some(cost) = can_go_cost {
                        let step = Step::new(start, end, cost);
                        new_steps.push(step);
                    }
                }

                let mut anphipod_in_correct_room = i == room;

                if anphipod_in_correct_room {
                    for j in init_row..total_rows {
                        if self.fields[j][i] != space {
                            anphipod_in_correct_room = false;
                            break;
                        }
                    }
                }

                if !anphipod_in_correct_room {
                    let start = Position{ row: init_row, col: i };

                    // Testing if an anphipod can go to the hallway
                    for j in STAYING_POSITIONS {
                        if self.fields[0][j] == '.' {
                            let end = Position{ row: 0, col: j };
                            let can_go_cost = self.anphipod_can_go_to(&start, &end);

                            if let Some(cost) = can_go_cost {
                                let step = Step::new(start, end, cost);
                                new_steps.push(step);
                            }
                        }
                    }
                }
            }
        }

        new_steps
    }

    // All anphipod are the correct position
    fn check(&self) -> bool {
        let total_rows = self.fields.len();

        for i in (2..10).step_by(2) {
            let anphipod = anphipod_by_room(i);

            for j in 1..total_rows {
                if self.fields[j][i] != anphipod {
                    return false;
                }
            }
        }

        return true;
    }

    fn exec(&mut self) -> Option<usize> {
        let next_steps_idx = {
            let joined = self.fields_to_string();

            if !self.tracked.contains_key(&joined) {
                let idx = self.tracked_stack.len();
                self.tracked.insert(joined.clone(), idx);

                let new_steps = self.eval_step();
                self.tracked_stack.push(new_steps);
                self.tracked_stack_costs.push(None);

                idx
            } else {
                *self.tracked.get(&joined).unwrap()
            }
        };

        if self.tracked_stack[next_steps_idx].len() == 0 {
            if self.check() {

                if self.min_cost > self.current_path.cost {
                    println!("path cost: {}", self.current_path.cost);
                    self.min_cost = self.current_path.cost;
                    self.min_cost_path = Some(self.current_path.clone());
                }

                return Some(0);
            }

            return None;
        }

        let mut it_min_cost = self.tracked_stack_costs[next_steps_idx];

        if let Some(min_tracked_cost) = it_min_cost {
            if self.current_path.cost + min_tracked_cost > self.min_cost {
                return it_min_cost;
            }
        }

        let size = self.tracked_stack[next_steps_idx].len();

        for i in (0..size).rev() {
            let step = self.tracked_stack[next_steps_idx][i];

            self.fields_swap(&step.start, &step.end);

            self.current_path.cost += step.cost;
            self.current_path.steps.push(step); 

            if let Some(new_min) = self.exec() {
                if let Some(min_tracked_cost) = it_min_cost {
                    if min_tracked_cost > new_min + step.cost {
                        it_min_cost = Some(new_min + step.cost);
                    }
                } else {
                    it_min_cost = Some(new_min + step.cost);
                }
            } else {
                self.tracked_stack[next_steps_idx].swap_remove(i);
            }

            self.current_path.cost -= step.cost;
            self.current_path.steps.pop(); 

            self.fields_swap(&step.start, &step.end);
        }

        self.tracked_stack_costs[next_steps_idx] = it_min_cost;

        return it_min_cost;
    }
}

fn part1() {
    // let fields: Vec<[char; 11]> = vec![
    //     //  0    1    2    3    4    5    6    7    8    9   10
    //     ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
    //     [' ', ' ', 'B', ' ', 'C', ' ', 'B', ' ', 'D', ' ', ' '],
    //     [' ', ' ', 'A', ' ', 'D', ' ', 'C', ' ', 'A', ' ', ' '],
    // ];

    let fields: Vec<[char; 11]> = vec![
        //  0    1    2    3    4    5    6    7    8    9   10
        ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        [' ', ' ', 'D', ' ', 'D', ' ', 'C', ' ', 'C', ' ', ' '],
        [' ', ' ', 'B', ' ', 'A', ' ', 'B', ' ', 'A', ' ', ' '],
    ];

    let mut manager = PathManager::new(fields);

    manager.exec();

    manager.follow_path();

    println!("min cost: {}", manager.min_cost);
}

fn part2() {
    // let fields: Vec<[char; 11]> = vec![
    //     //  0    1    2    3    4    5    6    7    8    9   10
    //     ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
    //     [' ', ' ', 'B', ' ', 'C', ' ', 'B', ' ', 'D', ' ', ' '],
    //     [' ', ' ', 'D', ' ', 'C', ' ', 'B', ' ', 'A', ' ', ' '],
    //     [' ', ' ', 'D', ' ', 'B', ' ', 'A', ' ', 'C', ' ', ' '],
    //     [' ', ' ', 'A', ' ', 'D', ' ', 'C', ' ', 'A', ' ', ' '],
    // ];

    let fields: Vec<[char; 11]> = vec![
        //  0    1    2    3    4    5    6    7    8    9   10
        ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        [' ', ' ', 'D', ' ', 'D', ' ', 'C', ' ', 'C', ' ', ' '],
        [' ', ' ', 'D', ' ', 'C', ' ', 'B', ' ', 'A', ' ', ' '],
        [' ', ' ', 'D', ' ', 'B', ' ', 'A', ' ', 'C', ' ', ' '],
        [' ', ' ', 'B', ' ', 'A', ' ', 'B', ' ', 'A', ' ', ' '],
    ];

    let mut manager = PathManager::new(fields);

    manager.exec();

    manager.follow_path();

    println!("min cost: {}", manager.min_cost);
}

fn main() {
    // part1();
    part2();
}
