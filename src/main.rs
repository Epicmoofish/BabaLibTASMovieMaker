#[macro_use]
extern crate fstrings;

use std::fmt::{Display, Formatter};

struct Instruction {
    instruct: Vec<String>,
    frames: Vec<Vec<String>>,
    framecount: usize,
}
impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for i in &self.frames {
            let k = i.join(" ");
            str = f!("{str} |{k}|");
        }
        write!(f, "{}", str[1..].to_string())
    }
}
impl Instruction {
    fn new(str: String, fast: bool) -> Instruction {
        let expand = || -> Vec<String> {
            let mut expanded: Vec<String> = Vec::new();
            let instructlen = str.split(" ");
            for inst in instructlen {
                let res = inst[1..].parse::<u16>();
                if res.is_ok() {
                    if inst.eq("F2") {
                        expanded.push("F2".to_string());
                    } else {
                        for _ in 0..res.unwrap() {
                            let mut k = String::new();
                            inst[0..=0].clone_into(&mut k);
                            expanded.push(k);
                        }
                    }
                } else {
                    expanded.push(inst.to_string());
                }
            }
            return expanded;
        };
        let expanded = expand();
        let frames = || -> Vec<Vec<String>> {
            let mut frames: Vec<Vec<String>> = Vec::new();
            let mut frameind = 0;
            let mut allowed = 2;
            frames.push(Vec::new());
            for i in &expanded {
                if i == "W" {
                    allowed = 2 - frames[frameind].len();
                    frames[frameind].push(i.clone());
                    frameind+=1;
                    frames.push(Vec::new());
                } else if i.len() == 1 {
                    while frames[frameind].len() >= allowed {
                        allowed = 2 - frames[frameind].len();
                        frameind += 1;
                        frames.push(Vec::new());
                    }
                    frames[frameind].push(i.clone());
                } else {
                    if frames[frameind].len() > 0 {
                        frameind += 1;
                        frames.push(Vec::new());
                    }
                    let str: &str = i;
                    match str {
                        "Reset" => {
                            // println!("Hello");
                        }
                        &_ => {}
                    }
                }
            }
            return frames;
        };
        let frame_vec = frames();
        let frame_vec_len = frame_vec.len();
        return Instruction {
            instruct: expanded,
            frames: frame_vec,
            framecount: frame_vec_len
        };
    }
}
struct DataFrame {
    vec: Vec<Vec<String>>
}
impl DataFrame {
    fn get_values(&self, key_p: &str) -> Option<Vec<String>>{
        let key = key_p.to_string();
        let mut vec: Vec<String> = Vec::new();
        let mut index = 0;
        for i in &self.vec[1] {
            if i.eq(&key) {
                let allbut = &self.vec[2..];
                for j in allbut{
                    vec.push(j[index].clone());
                }
                return Some(vec);
            }
            index += 1;
        }
        return None;
    }
    fn get_moveset_str(&self, level: &str) -> Option<String> {
        let lvl = level.to_string();
        let names = self.get_values("Individual Levels Name")?;
        let movesets = self.get_values("Individual Levels Moveset")?;
        for ind in 0..names.len() {
            let name = names[ind].clone();
            if lvl.eq(&name) {
                if movesets[ind].is_empty() {
                    return None
                }
                return Some(movesets[ind].clone())
            }
        }
        return None;
    }

    fn clean(&mut self) {
        let mut last = String::new();
        let mut cleanall = false;
        let mut last_header = String::new();
        for a in 0..self.vec[0].len() {
            if !self.vec[0][a].is_empty() {
                last_header = self.vec[0][a].clone();
            }
            self.vec[1][a] = last_header.clone() + " " + &*self.vec[1][a].clone();
        }
        for index in 2..self.vec.len() {
            let i = self.vec[index].clone();
            if cleanall {
                self.vec[index][0] = String::new();
                self.vec[index][1] = String::new();
                self.vec[index][2] = String::new();
                self.vec[index][3] = String::new();
                self.vec[index][4] = String::new();
            }
            else {
                let mut modified = false;
                if !i[0].is_empty() {
                    last = i[0].clone();
                }
                if !i[1].is_empty() || i[0].is_empty() {
                    if !i[1].eq("Bonus") {
                        let clonedlast = last.clone();
                        let clonedi1 = i[1].clone();
                        self.vec[index][0] = f!("{clonedlast} ({clonedi1})");
                        modified = true;
                    }
                }
                if index+1 != self.vec.len() && !modified {
                    if self.vec[index + 1][0].is_empty() && !self.vec[index][0].is_empty() {
                        self.vec[index][0] = i[0].clone() + " (Win)";
                    }
                }
                if self.vec[index][0].eq("The End (Done)") {
                    cleanall = true;
                }
            }
        }
    }
}
fn scrape_url(url: String) -> DataFrame {
    let response = reqwest::blocking::get(url);
    let html_content = response.unwrap().text().unwrap();
    let splitterfirst = "\r\n";
    let splitter2nd = ",";
    let n = html_content.split(splitterfirst);
    let mut vec:Vec<Vec<String>> = Vec::new();
    for i in n {
        let mut newvec : Vec<String> = Vec::new();
        for j in i.split(splitter2nd) {
            newvec.push(j.to_string().replace("\n", ""));
        }
        vec.push(newvec);
    }

    return DataFrame {vec};
}
fn main() {
    //Default Before: MOVETONEXT
    //Default After: ENTERLEVEL

    let sheet_id = "1nICIicSDPCreqlegC0uQs0_tUveeruKaAxRKV2tUhYI";
    let sheet_gid = "1044476473";
    let doc_url = f!("https://docs.google.com/spreadsheets/d/{sheet_id}/export?format=csv&id=1nICIicSDPCreqlegC0uQs0_tUveeruKaAxRKV2tUhYI&gid={sheet_gid}");
    let mut k = scrape_url(doc_url.clone());
    k.clean();
    println!("{:?}", k.vec);
    let opt = k.get_values("Map Movement and Routing To");
    if opt.is_some() {
        for i in opt.unwrap() {
            let v = k.get_moveset_str(i.as_str());
            if v.is_some() {
                let k = Instruction::new(v.unwrap(), false);
                println!("{}: {}", i, k.framecount);
            }
        }
    }
    // let k = Instruction::new("U4 R3 U2 L1 U1 R1 D4 L3 U1 L1 D6 U4 R5 U4 L1 U1 W3 R4 L4 D6 L1 D2 L7 U3 D1 W2 D2 W2 R2 W2 R1 U1 W2 R2 W2 R2 W1 R3 U7 W2 R2 D3 U3 L5 D7 R7 D1 R1 U2 D1 R2 U2 L5 U3 L2 U2 R2 D4 U4 L6 D10 L1 D3 L4 D1 R5 D1 R1 U2 L1 U1 R2 L3 U2 L1 U4 Reset".to_string());
    // println!("{}", k);
}
