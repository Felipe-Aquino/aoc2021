use std::fs;

#[derive(PartialEq, Eq)]
enum CaveType {
    Small,
    Big,
    Start,
    End,
}

struct Cave {
    id: usize,
    name: String,
    cave_type: CaveType,

    neighbors: Vec<usize>,

    visit_count: usize,
}

impl Cave {
    fn new(id: usize, name: &str) -> Self {
        let cave_type =
            if name == "start" {
                CaveType::Start
            } else if name == "end" {
                CaveType::End
            } else {
                let c = name.chars().nth(0).unwrap();

                if c.is_lowercase() {
                    CaveType::Small
                } else {
                    CaveType::Big
                }
            };

        Cave {
            id,
            name: name.to_string(),
            cave_type,
            neighbors: vec![],
            visit_count: 0,
        }
    }

    fn is_small(&self) -> bool {
        self.cave_type != CaveType::Big
    }

    fn add_neighbor(&mut self, cave_idx: usize) {
        let found = self.neighbors
            .iter()
            .find(|&&id| id == cave_idx);

        if found.is_none() {
            self.neighbors.push(cave_idx);
        }
    }

    fn add_visit(&mut self) {
        self.visit_count += 1;
    }
}

struct State {
    caves: Vec<Cave>,

    current_path: Vec<usize>,
    paths: Vec<Vec<usize>>,

    start_idx: usize,
    end_idx: usize,

    is_the_end: bool,

    max_visit_count_once: usize,
    max_visit_reached: bool,
}

impl State {
    fn new(max_visit_count_once: usize) -> State {
        State {
            caves: vec![],

            current_path: vec![],
            paths: vec![],

            start_idx: 0,
            end_idx: 0,

            is_the_end: false,
            max_visit_count_once,
            max_visit_reached: false,
        }
    }

    fn find_cave(&mut self, name: &str) -> usize {
        for cave in self.caves.iter() {
            if cave.name.as_str() == name {
                return cave.id;
            }
        }

        let cave = Cave::new(self.caves.len(), name);

        self.caves.push(cave);

        self.caves.len() - 1
    }

    fn mount_paths_for(&mut self, cave_idx: usize, amount: usize) {
        let cave = &mut self.caves[cave_idx];

        if cave_idx == self.start_idx {
            if cave.visit_count > 0 {
                return;
            }
        }

        self.current_path.push(cave_idx);

        if cave_idx == self.end_idx {
            self.is_the_end = true;
            return;
        }

        if cave.is_small() {
            cave.add_visit();

            self.max_visit_reached |= cave.visit_count >= self.max_visit_count_once;
        }

        let path_size = self.current_path.len();

        for n in cave.neighbors.clone().iter() {
            let v = *n;

            let ok =
                if !self.max_visit_reached {
                    self.caves[v].visit_count < self.max_visit_count_once
                } else {
                    self.caves[v].visit_count == 0
                };

            if ok {
                self.mount_paths_for(v, amount + 1);

                if self.is_the_end {
                    self.paths.push(self.current_path.clone());
                    self.is_the_end = false;
                }

                for i in path_size..self.current_path.len() {
                    let j = self.current_path[i];
                    if self.caves[j].visit_count > 0 {
                        self.caves[j].visit_count -= 1;
                    }
                }

                self.current_path.drain(path_size..);

                self.max_visit_reached = false;

                for i in 0..path_size {
                    let j = self.current_path[i];

                    if self.caves[j].visit_count >= self.max_visit_count_once {
                        self.max_visit_reached = true;
                        break;
                    }
                }
            }
        }
    }

    fn init(&mut self, paths: &Vec<(&str, &str)>) {
        for &(s, e) in paths.iter() {
            let fst_idx = self.find_cave(s);
            let snd_idx = self.find_cave(e);

            self.caves[fst_idx].add_neighbor(snd_idx);
            self.caves[snd_idx].add_neighbor(fst_idx);
        }

        for cave in self.caves.iter() {
            if cave.cave_type == CaveType::Start {
                self.start_idx = cave.id;
            } else if cave.cave_type == CaveType::End {
                self.end_idx = cave.id;
            }
        }

        self.mount_paths_for(self.start_idx, 0);
    }

    #[allow(dead_code)]
    fn print_paths(&self) {
        for path in self.paths.iter() {
            for n in path.iter() {
                print!("{}-", self.caves[*n].name);
            }
            print!("\n");
        }
    }

    fn print_total_paths(&self) {
        println!("paths: {}", self.paths.len());
    }
}

#[allow(dead_code)]
fn part1(content: String) {
    let paths: Vec<(&str, &str)> = content
        .lines()
        .map(|line| {
            let splited = line
                .split('-')
                .collect::<Vec<&str>>();

            (splited[0], splited[1])
        })
        .collect();

    let mut state = State::new(1);

    state.init(&paths);

    // state.print_paths();
    state.print_total_paths();
}

fn part2(content: String) {
    let paths: Vec<(&str, &str)> = content
        .lines()
        .map(|line| {
            let splited = line
                .split('-')
                .collect::<Vec<&str>>();

            (splited[0], splited[1])
        })
        .collect();

    let mut state = State::new(2);

    state.init(&paths);

    // state.print_paths();
    state.print_total_paths();
}

fn main() {
    // let filename = "./inputs/day12-example1.txt";
    // let filename = "./inputs/day12-example2.txt";
    // let filename = "./inputs/day12-example3.txt";
    let filename = "./inputs/day12.txt";

    let content = fs::read_to_string(filename)
        .expect("Could not read file");

    part2(content);
}
