extern crate colored;

use colored::*;

const EDGE_SIZE : i8 = 4;

// scaled coordinates
fn dummy(x: i8, y: i8) -> bool { (x.abs() + y.abs()) % 2 == 1 }
fn cond_a1(x: i8, y: i8) -> bool { x - y >= -EDGE_SIZE * 2 }
fn cond_a2(x: i8, y: i8) -> bool { x - y <= EDGE_SIZE * 2 }
fn cond_b1(x: i8, y: i8) -> bool { x + y >= -EDGE_SIZE * 2 }
fn cond_b2(x: i8, y: i8) -> bool { x + y <= EDGE_SIZE * 2 }
fn cond_c1(_x: i8, y: i8) -> bool { y >= -EDGE_SIZE }
fn cond_c2(_x: i8, y: i8) -> bool { y <= EDGE_SIZE }

fn main() {
    const WIDTH : i8 = (EDGE_SIZE * 3 + 1) * 2 + 1;
    const HEIGHT : i8 = (EDGE_SIZE * 4 + 1) + 2;

    let mut map: [[i8; WIDTH as usize]; HEIGHT as usize] = unsafe { std::mem::uninitialized() };
    let offset_y : i8 = map.len() as i8 / 2;
    let offset_x : i8 = map[0].len() as i8 / 2;
    for y in -offset_y..offset_y+1 {
        for x in -offset_x..offset_x+1 {
            // boundary conditions
            let trues = [ dummy(x, y),
                          cond_a1(x, y), cond_a2(x, y),
                          cond_b1(x, y), cond_b2(x, y),
                          cond_c1(x, y), cond_c2(x, y) ];

            // board labels
            map[(y + offset_y) as usize][(x + offset_x) as usize] =
                match trues {
                    // dummy
                    [true, _, _, _, _, _, _] => -1,

                    // middle
                    [_, true, true, true, true, true, true] => 9,

                    // sources and destinations
                    [_, false, true, true, true, true, true] => 1,
                    [_, true, false, true, true, true, true] => 2,
                    [_, true, true, false, true, true, true] => 3,
                    [_, true, true, true, false, true, true] => 4,
                    [_, true, true, true, true, false, true] => 5,
                    [_, true, true, true, true, true, false] => 6,

                    // ignore
                    _ => 0,
                };
        }
    }

    // print
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let value = map[i][j];

            // colored output
            let output = match value {
                9 => "0".white(),
                1 | 2 => value.to_string().red(),
                3 | 4 => value.to_string().green(),
                5 | 6 => value.to_string().yellow(),
                _ => " ".black(),
            };
            print!("{} ",output);
        }
        println!();
    }

}
