use std::env;

#[derive(PartialEq)]
enum State {
    Dead,
    Alive,
    None,
}

struct Pos {
    x:usize,
    y:usize,
    s:State,
}

fn parse_args(args: Vec<String>) -> (String, String){
    if args.len() < 3 {
        panic!("Not enough args");
    }

    let x = &args[1];
    let y = &args[2];
    (x.to_string(), y.to_string())
}

fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn init(x_axis:u64 , y_axis:u64) -> Vec<Vec<Pos>>{
    let mut cells:Vec<Vec<Pos>> = Vec::new();
    for _ in 0..y_axis {
        cells.push(Vec::new());
    }
    let mut i : usize = 0;

    for cell in &mut cells {
        for j in 0..x_axis {
            cell.push(Pos{x : i, y : j as usize, s : State::None});
        }
        i += 1;
    }
    cells[(y_axis/2 - 1) as usize][ (x_axis/2 + 1) as usize].s = State::Alive; 
    cells[(y_axis/2 - 1) as usize][ (x_axis/2 + 2) as usize].s = State::Alive; 
    cells[(y_axis/2) as usize][ (x_axis/2 + 1) as usize].s = State::Alive; 
    cells[(y_axis/2) as usize][ (x_axis/2 + 2) as usize].s = State::Alive; 
    cells[(y_axis/2 + 1) as usize][ (x_axis/2 + 1) as usize].s = State::Alive; 
    cells[(y_axis/2 + 1) as usize][ (x_axis/2 + 2) as usize].s = State::Alive; 

    cells
}

fn print_cells(cells:&Vec<Vec<Pos>>) {
    for cell in cells {
        for p in cell {
            let out = match p.s {
                State::Dead => ' ',
                State::Alive => '#',
                State::None => ' ',
            };
            print!("{}", out);
        }
        print!("\n");
    }
}

fn cgol(x_axis:u64, y_axis:u64) {
    let mut cells = init(x_axis, y_axis);
    let x_axis = x_axis as usize;
    let y_axis = y_axis as usize;

    loop {
        for i in 0..y_axis {
        //for cell in &cells {
            //for p in cell {
            for j in 0..y_axis {
                let p = &cells[i][j];
                if p.s == State::None {
                    continue;
                }

                let under = if p.y == 0 { y_axis - 1 } else { p.y - 1};
                let above = if p.y == y_axis - 1 { 0 } else { p.y + 1};
                let left = if p.x == 0 { x_axis - 1 } else { p.x - 1};
                let right = if p.x == x_axis - 1 { 0 } else { p.x + 1};
                let mut alive : u64 = 0;

                if cells[above][left].s == State::Alive { alive+=1; }
                if cells[above][p.x].s == State::Alive { alive+=1; }
                if cells[above][right].s == State::Alive { alive+=1; }

                if cells[i][left].s == State::Alive { alive+=1; }
                if cells[i][right].s == State::Alive { alive+=1; }

                if cells[under][left].s == State::Alive { alive+=1; }
                if cells[under][p.x].s == State::Alive { alive+=1; }
                if cells[under][right].s == State::Alive { alive+=1; }

                let p = &mut cells[i][j];
                match p.s {
                     State::Alive => {
                        if alive < 2 || alive > 3 {
                            p.s = State::Dead;
                        }else {
                            p.s = State::Alive;
                        }                    },
                    State::Dead => {
                        if alive == 3 {
                            p.s = State::Alive;
                        }
                    },
                    State::None => {panic!("Should never happen");},
                }
            }
        }
        clear();
        print_cells(&cells);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (x_string, y_string) = parse_args(args);

    let x_axis : i64 = match x_string.parse() {
        Ok(val) => if val <= 0 { panic!("X axis should be positive and larger than 0") } else {val},
        Err(err) => panic!("Failed to convert with err {err:?}"),
    };

    let y_axis : i64 = match y_string.parse() {
        Ok(val) => if val <= 0 { panic!("Y axis should be positive and larger than 0") } else {val},
        Err(err) => panic!("Failed to convert with err {err:?}"),
    };

    cgol(x_axis.try_into().unwrap(), y_axis.try_into().unwrap());
}
