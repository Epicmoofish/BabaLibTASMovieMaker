#[macro_use]
extern crate fstrings;
#[derive(Debug)]
struct Instruction {
    instruct: Vec<String>
}
impl Instruction {
    fn new(str: String) -> Instruction {
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
        return Instruction {
            instruct: expanded
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
        for i in &self.vec[0] {
            if i.eq(&key) {
                let allbut = &self.vec[1..];
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
        let movesets = self.get_values("Moveset")?;
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
        for index in 1..self.vec.len() {
            let i = self.vec[index].clone();
            if cleanall {
                self.vec[index][0] = String::new();
                self.vec[index][1] = String::new();
                self.vec[index][2] = String::new();
                self.vec[index][3] = String::new();
                self.vec[index][4] = String::new();
            }
            else {
                if !i[0].is_empty() {
                    last = i[0].clone();
                }
                if !i[1].is_empty() || i[0].is_empty() {
                    if !i[1].eq("Bonus") {
                        self.vec[index][0] = last.clone() + " (" + &*i[1].clone() + ")";
                    }
                }
                if index+1 != self.vec.len() {
                    if self.vec[index + 1][0].is_empty() && !self.vec[index].is_empty() {
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
    let splitterfirst = "\"\n\"";
    let splitter2nd = "\",\"";
    let n = html_content.split(splitterfirst);
    let mut vec:Vec<Vec<String>> = Vec::new();
    for i in n {
        let mut newvec : Vec<String> = Vec::new();
        for j in i.split(splitter2nd) {
            newvec.push(j.to_string());
        }
        vec.push(newvec);
    }
    let lastone = vec.len()-1;
    let lasttwo = vec[lastone].len()-1;
    vec[0][0] = vec[0][0][1..].to_string();
    vec[lastone][lasttwo] = vec[lastone][lasttwo][0..vec[lastone][lasttwo].len()-1].to_string();

    return DataFrame {vec};
}
fn main() {
    let sheet_id = "1nICIicSDPCreqlegC0uQs0_tUveeruKaAxRKV2tUhYI";
    let level_movesets = "TAS%20Info";
    let doc_url = f!("https://docs.google.com/spreadsheets/d/{sheet_id}/gviz/tq?tqx=out:csv&sheet={level_movesets}");
    let mut k = scrape_url(doc_url.clone());
    k.clean();
    let opt = k.get_values("Map Movement and Routing To");
    if opt.is_some() {
        for i in opt.unwrap() {
            let v = k.get_moveset_str(i.as_str());
            if v.is_some() {
                println!("{}: {}", i, v.unwrap());
            }
        }
    }
    let inst = Instruction::new("F2".to_string());
    println!("{:?}", inst.instruct);
}
