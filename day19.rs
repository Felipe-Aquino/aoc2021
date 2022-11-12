use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn print_orientation_value(value: i32) {
    if value == 1 {
        print!("+x");
    } else if value == -1 {
        print!("-x");
    } else if value == 2 {
        print!("+y");
    } else if value == -2 {
        print!("-y");
    } else if value == 3 {
        print!("+z");
    } else if value == -3 {
        print!("-z");
    } else {
        print!("unknown");
    }
}

fn print_orientation(ori: &(i32, i32, i32)) {
    print_orientation_value(ori.0);
    print!(", ");
    print_orientation_value(ori.1);
    print!(", ");
    print_orientation_value(ori.2);
    print!("\n");
}

fn from_orientation_value(coord: &(i32, i32, i32), ori_value: i32) -> i32 {
    if ori_value == 1 {
        return coord.0;
    } else if ori_value == -1 {
        return -coord.0;
    } else if ori_value == 2 {
        return coord.1;
    } else if ori_value == -2 {
        return -coord.1;
    } else if ori_value == 3 {
        return coord.2;
    } else if ori_value == -3 {
        return -coord.2;
    } else {
        unreachable!();
    }
}

fn sum_coords(a: &(i32, i32, i32), b: &(i32, i32, i32)) -> (i32, i32, i32) {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn from_orientation(coord: &(i32, i32, i32), ori: &(i32, i32, i32)) -> (i32, i32, i32) {
    let x = from_orientation_value(&coord, ori.0);
    let y = from_orientation_value(&coord, ori.1);
    let z = from_orientation_value(&coord, ori.2);

    (x, y, z)
}

fn read_scanners(content: String) -> Vec<Vec<(i32, i32, i32)>> {
    let mut scanners_beacons: Vec<Vec<(i32, i32, i32)>> = vec![];
    let mut current_scanner = -1;

    for line in content.lines() {
        let _line = line.trim();

        if _line.len() == 0 {
            continue;
        }

        if _line.starts_with("--- ") {
            current_scanner += 1;
            scanners_beacons.push(vec![]);
            continue;
        }


        let coords = _line
            .split(",")
            .map(|num_str| {
                i32::from_str_radix(num_str, 10).unwrap()
            })
            .collect::<Vec<i32>>();

        scanners_beacons[current_scanner as usize].push((coords[0], coords[1], coords[2]));
    }

    return scanners_beacons;
}

// Position of scanner2 relative to scanner1
fn find_relative_position(
    orientations: &Vec<(i32, i32, i32)>,
    scanner1: &Vec<(i32, i32, i32)>,
    scanner2: &Vec<(i32, i32, i32)>,
) -> Option<(i32, (i32, i32, i32))> {

    for (ori_idx, ori) in orientations.iter().enumerate() {
        let mut found_rel_positions_count: HashMap<(i32, i32, i32), i32> = HashMap::new();

        for i in 0..scanner1.len() {
            let (p0x, p0y, p0z) = scanner1[i];

            for j in 0..scanner2.len() {
                let (p1x, p1y, p1z) = from_orientation(&scanner2[j], &ori);

                let rel_position = (p0x - p1x, p0y - p1y, p0z - p1z); 

                if let Some(count) = found_rel_positions_count.get_mut(&rel_position) {
                    *count += 1;

                    if *count == 12 {
                        print_orientation(&ori);
                        println!("{:?}: {}", rel_position, *count);

                        return Some((ori_idx as i32, rel_position));
                    }
                } else {
                    found_rel_positions_count.insert(rel_position, 1);
                }
            }
        }

    }

    None
}

fn manhattan_distance(a: &(i32, i32, i32), b: &(i32, i32, i32)) -> i32 {
    (b.0 - a.0).abs() + (b.1 - a.1).abs() + (b.2 - a.2).abs()
}

fn part1(content: String) {
    let mut scanners_beacons = read_scanners(content);

    let x_idx: i32 = 1;
    let y_idx: i32 = 2;
    let z_idx: i32 = 3;

    let orientations: Vec<(i32, i32, i32)> = vec![
        ( x_idx,  y_idx,  z_idx),
        (-y_idx,  x_idx,  z_idx),
        (-x_idx, -y_idx,  z_idx),
        ( y_idx, -x_idx,  z_idx),

        (-x_idx,  y_idx, -z_idx),
        (-y_idx, -x_idx, -z_idx),
        ( x_idx, -y_idx, -z_idx),
        ( y_idx,  x_idx, -z_idx),

        ( z_idx,  y_idx, -x_idx),
        ( z_idx, -y_idx,  x_idx),
        ( z_idx,  x_idx,  y_idx),
        ( z_idx, -x_idx, -y_idx),

        (-z_idx, -y_idx, -x_idx),
        (-z_idx,  y_idx,  x_idx),
        (-z_idx, -x_idx,  y_idx),
        (-z_idx,  x_idx, -y_idx),

        (-y_idx,  z_idx, -x_idx),
        ( y_idx,  z_idx,  x_idx),
        (-x_idx,  z_idx,  y_idx),
        ( x_idx,  z_idx, -y_idx),

        ( y_idx, -z_idx, -x_idx),
        (-y_idx, -z_idx,  x_idx),
        ( x_idx, -z_idx,  y_idx),
        (-x_idx, -z_idx, -y_idx),
    ];

    let mut scanners_positions: Vec<(i32, i32, i32)> = vec![];

    for _ in 0..scanners_beacons.len() {
        scanners_positions.push((0, 0, 0));
    }

    let mut scanners_done = vec![false; scanners_beacons.len()];

    scanners_done[0] = true;

    let mut i = 0;
    let mut all_done = false;

    while !all_done {
        if scanners_done[i] {
            for j in 0..scanners_beacons.len() {
                if scanners_done[j] {
                    println!("({}, {}) alreary done", i, j);
                    continue;
                }

                println!("({}, {})", i, j);

                let r = find_relative_position(
                    &orientations,
                    &scanners_beacons[i],
                    &scanners_beacons[j]
                );

                if let Some((ori_idx, rel)) = r {
                    scanners_done[j] = true;

                    for k in 0..scanners_beacons[j].len() {
                        let beacon_pos = from_orientation(
                            &scanners_beacons[j][k],
                            &orientations[ori_idx as usize]
                        );

                        scanners_beacons[j][k] = sum_coords(&beacon_pos, &rel);
                    }

                    scanners_positions[j] = rel;
                }
            }
        }

        i = (i + 1) % scanners_beacons.len();

        if i == 0 {
            all_done = true;

            for k in 0..scanners_done.len() {
                if !scanners_done[k] {
                    all_done = false;
                    break;
                }
            }
        }
    }

    let mut beacons_set = HashSet::new();

    for scanner in scanners_beacons {
        for beacon in scanner {
            beacons_set.insert(beacon.clone());
        }
    }

    let mut max_distance: i32 = 0;

    for j in 0..scanners_positions.len() {
        for k in j+1..scanners_positions.len() {
            let d = manhattan_distance(&scanners_positions[j], &scanners_positions[k]);

            if d > max_distance {
                max_distance = d;
            }
        }

    }

    /*
    beacons_set
        .iter()
        .for_each(|b| {
            println!("{:?}", b);
        });
    */

    println!("count: {}", beacons_set.len());
    println!("max manhattan distance: {}", max_distance);
}

fn main() {
    // let filename = "./inputs/day19-example.txt";
    let filename = "./inputs/day19.txt";

    let data = fs::read_to_string(filename)
        .expect("Could not read file");

    part1(data);
}

/*
| 1  0  0|
| 0  1  0|
| 0  0  1|
.....................
| 0 -1  0|
| 1  0  0|
| 0  0  1|
.....................
|-1  0  0|
| 0 -1  0|
| 0  0  1|
.....................
| 0  1  0|
|-1  0  0|
| 0  0  1|

.....................
|-1  0  0|
| 0  1  0|
| 0  0 -1|
.....................
| 0 -1  0|
|-1  0  0|
| 0  0 -1|
.....................
| 1  0  0|
| 0 -1  0|
| 0  0 -1|
.....................
| 0  1  0|
| 1  0  0|
| 0  0 -1|

.....................
| 0  0  1|
| 0  1  0|
|-1  0  0|
.....................
| 0  0  1|
| 0 -1  0|
| 1  0  0|
.....................
| 0  0  1|
| 1  0  0|
| 0  1  0|
.....................
| 0  0  1|
|-1  0  0|
| 0 -1  0|

.....................
| 0  0 -1|
| 0 -1  0|
|-1  0  0|
.....................
| 0  0 -1|
| 0  1  0|
| 1  0  0|
.....................
| 0  0 -1|
|-1  0  0|
| 0  1  0|
.....................
| 0  0 -1|
| 1  0  0|
| 0 -1  0|

.....................
| 0 -1  0|
| 0  0  1|
|-1  0  0|
.....................
| 0  1  0|
| 0  0  1|
| 1  0  0|
.....................
|-1  0  0|
| 0  0  1|
| 0  1  0|
.....................
| 1  0  0|
| 0  0  1|
| 0 -1  0|

.....................
| 0  1  0|
| 0  0 -1|
|-1  0  0|
.....................
| 0 -1  0|
| 0  0 -1|
| 1  0  0|
.....................
| 1  0  0|
| 0  0 -1|
| 0  1  0|
.....................
|-1  0  0|
| 0  0 -1|
| 0 -1  0|
.....................
*/
