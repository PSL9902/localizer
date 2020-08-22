#![allow(non_upper_case_globals, unused_imports, dead_code)]
extern crate localizer;
use std::time::{self, Duration, SystemTime};

static let_x: &str = "    
\\  /
 \\/
 /\\
/  \\

";
static let_0: &str = "
   0   
 0   0 
0     0
 0   0 
   0   
";

struct GameObj {
    table: Vec<u8>,
}
impl GameObj {
    fn new() -> Self {
        let mut tab = Vec::new();
        for _i in 0..9 {
            tab.push(0);
        }
        Self { table: tab }
    }
    fn printmach(&self) -> Vec<u8> {
        self.table.clone()
    }
    fn retfreecells(&self) -> Vec<u8> {
        let mut res = Vec::new();
        for (_num, _i) in self.table.iter().enumerate() {
            if *_i == 0 {
                res.push(_num as u8);
            }
        }
        res
    }
    fn who_win(&self) -> u8 {
        let win_pos = vec![
            (0, 1, 2),
            (3, 4, 5),
            (6, 7, 8),
            (0, 3, 6),
            (1, 4, 7),
            (2, 5, 8),
            (0, 4, 8),
            (2, 4, 6),
        ];
        for _i in &win_pos {
            if self.table[_i.0] == self.table[_i.1]
                && self.table[_i.1] == self.table[_i.2]
                && self.table[_i.2] != 0
            {
                return self.table[_i.0];
            }
        }
        let mut count = 0;
        let mut y = Vec::new();
        for _i in win_pos {
            y.push(
                (self.table[_i.0] == self.table[_i.1]
                    || self.table[_i.0] == 0
                    || self.table[_i.1] == 0)
                    && (self.table[_i.1] == self.table[_i.2]
                        || self.table[_i.1] == 0
                        || self.table[_i.2] == 0)
                    && (self.table[_i.0] == self.table[_i.2]
                        || self.table[_i.0] == 0
                        || self.table[_i.2] == 0),
            );
        }

        for _i in y {
            if _i == true {
                count += 1;
            }
        }
        if count <= 0 {
            return 3;
        }
        0
    }
    fn who_win_wteam(&self, team: u8) -> u8 {
        let win_pos = vec![
            (0, 1, 2),
            (3, 4, 5),
            (6, 7, 8),
            (0, 3, 6),
            (1, 4, 7),
            (2, 5, 8),
            (0, 4, 8),
            (2, 4, 6),
        ];
        for _i in &win_pos {
            if self.table[_i.0] == self.table[_i.1]
                && self.table[_i.1] == self.table[_i.2]
                && self.table[_i.2] != 0
            {
                return self.table[_i.0];
            }
        }
        let mut count = 0;
        let mut y = Vec::new();
        for _i in win_pos {
            let mut flag = true;
            flag = flag
                && ((self.table[_i.0] == team && self.table[_i.0] == self.table[_i.1])
                    || self.table[_i.0] == 0
                    || self.table[_i.1] == 0);
            flag = flag
                && ((self.table[_i.1] == team && self.table[_i.1] == self.table[_i.2])
                    || self.table[_i.1] == 0
                    || self.table[_i.2] == 0);
            flag = flag
                && ((self.table[_i.1] == team && self.table[_i.0] == self.table[_i.2])
                    || self.table[_i.0] == 0
                    || self.table[_i.2] == 0);
            y.push(flag); //(((self.table[_i.0] == team || self.table[_i.1]==team) && self.table[_i.0] == self.table[_i.1]) || self.table[_i.0] == 0 || self.table[_i.1] == 0) && (((self.table[_i.1] == team || team == self.table[_i.2]) && self.table[_i.1] == self.table[_i.2]) || self.table[_i.1] == 0 || self.table[_i.2] == 0)) && ((self.table[_i.0] == team || team == self.table[_i.2]) && (self.table[_i.0] == self.table[_i.2] || self.table[_i.0] == 0 || self.table[_i.2] == 0)));
        }

        for _i in y {
            if _i == true {
                count += 1;
            }
        }
        if count == 0 {
            return 3;
        }
        0
    }
    fn makemove(&mut self, numb: u8, team: u8) -> Result<String, String> {
        if self.who_win() == 0 {
            if numb <= 8 {
                if self.table[numb as usize] == 0 {
                    self.table[numb as usize] = team;
                } else {
                    return Err("".to_string());
                }
            } else {
                return Err("".to_string());
            }
        } else {
            return Err("".to_string());
        }
        Ok("".to_string())
    }
    fn print(&self) {
        println!("{}:", localizer::get_by_key1("table".to_string()));
        println!("=======");
        for (num, _i) in self.table.iter().enumerate() {
            if *_i == 1 {
                print!("|0");
                if (num + 1) % 3 == 0 {
                    println!("|");
                }
            } else if *_i == 2 {
                print!("|x");
                if (num + 1) % 3 == 0 {
                    println!("|");
                }
            } else/* if *_i==0*/ {
                print!("| ");
                if (num + 1) % 3 == 0 {
                    println!("|");
                }
            }
        }
        println!("=======");
    }
    fn new_print(&self) {
        println!("{}:", localizer::get_by_key1("table".to_string()));
        print!(" ");
        for _ in 0..34 {
            print!("=");
        }
        let _ = io::stdout().flush();
        println!();
        for _i in 0..3 {
            for _i1 in 0..6 {
                print!(" |");
                let _ = io::stdout().flush();
                for _i2 in 0..3 {
                    match self.table[_i * 3 + _i2 as usize] {
                        0 => {
                            /*let mut _st=let_space.lines();
                            let mut st = Vec::new();
                            while let Some(x) = _st.next()
                            {
                                st.push(x);
                            }*/
                            if _i1 == 3 {
                                print!("{:^10}", _i * 3 + _i2 + 1);
                            } else {
                                print!("{:^10}", " ");
                            }
                        }
                        1 => {
                            let mut _st = let_0.lines();
                            let mut st = Vec::new();
                            while let Some(x) = _st.next() {
                                st.push(x);
                            }
                            print!("{:^10}", st[_i1]);
                        }
                        2 => {
                            let mut _st = let_x.lines();
                            let mut st = Vec::new();
                            while let Some(x) = _st.next() {
                                st.push(x);
                            }
                            print!("{:^10}", st[_i1]);
                        }
                        _ => std::process::exit(100),
                    }
                    print!("|");
                    if _i2 == 2 {
                        println!();
                    }
                }
            }
            print!(" ");
            for _ in 0..34 {
                print!("=");
            }
            println!();
        }
        let _ = io::stdout().flush();
    }
}

fn newgame() {
    let mut ent = GameObj::new();
    let mut team = 0;
    while ent.who_win()/*_wteam(team+1)*/==0
    /*|| ent.who_win_wteam((1-team)+1)==0*/
    {
        //ent.print();
        ent.new_print();
        print!(
            "{:>50}",
            format!(
                "{}: {},       {}: ",
                localizer::get_by_key1("team".to_string()),
                ["0", "x"][team as usize],
                localizer::get_by_key1("input number".to_string())
            )
        );
        let _ = io::stdout().flush();
        let mut input = "".to_string();
        let _s = io::stdin().read_line(&mut input);
        input.retain(|c| !"\n\r".contains(c));
        let inputnum = input.parse::<u8>();
        if let Err(err) = {
            if let Ok(x) = inputnum {
                if x <= 9 && x >= 1 {
                    ent.makemove(x - 1, team + 1)
                } else {
                    Err("".to_string())
                }
            } else {
                Err("".to_string())
            }
        } {
            println!("{}: {}", localizer::get_by_key1("Error".to_string()), err);
            continue;
        }
        team = 1 - team;
    }
    if ent.who_win() != 0
    //who_win_wteam((1-team)+1)!=0
    {
        println!(
            "{}:{}",
            localizer::get_by_key1("win".to_string()),
            vec!["", "0", "x", &localizer::get_by_key1("nobody".to_string())][ent.who_win()/*_wteam((1-team)+1)*/ as usize]
        );
        ent.new_print();
    }
}

use std::io::{self, Stdout, Write};
use std::u8;
fn main() {
    print!("select language [0:\"ru\", 1:\"en\"]: ");
    std::io::stdout().flush().unwrap();
    let mut lang = String::new();
    std::io::stdin().read_line(&mut lang).unwrap();
    let lang = match lang.trim_end() {
        "0" | "ru" => "ru",
        _ => "en",
    };
    localizer::change_localizer(&|x| {
        x.set_current_lang(Some(lang.to_string()));
    });
    loop {
        newgame();
    }
}
