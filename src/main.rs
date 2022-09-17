use std::fs::File;
use std::io::{self, BufRead};
use std::iter::FromIterator;
use std::path::PathBuf;
extern crate rand;
extern crate walkdir;
use rand::Rng;
use walkdir::{DirEntry, WalkDir};

#[derive(Default)]
struct Raname {
    name: String,
    variants: Vec<String>,
}

// fn is_hidden(entry: &DirEntry) -> bool {
//     entry.file_name()
//          .to_str()
//          .map(|s| s.starts_with("."))
//          .unwrap_or(false)
// }

fn init() -> Vec<Raname> {
    let pth = std::fs::canonicalize(PathBuf::from("./")).unwrap();
    let wlkdir = WalkDir::new(pth /*.clone()*/);
    let wlkdir = wlkdir.max_depth(1);

    // for entry in wlkdir.into_iter().filter_entry(|e| !is_hidden(e)) {
    //     println!("{}", entry.unwrap().path().display());
    // }

    // let wlkdir = WalkDir::new(pth);
    // let wlkdir = wlkdir.max_depth(1);

    return Vec::from_iter(
        wlkdir
            .into_iter()
            .filter(|x| {
                x.as_ref()
                    .unwrap()
                    .path()
                    .extension()
                    .and_then(|y| if y == "dat" { Some(y) } else { None })
                    .is_some()
            })
            .map(|x| {
                match x {
                    Ok(path) => {
                        let filename = path.file_name();
                        let file = File::open(filename).unwrap();
                        let strs =
                            Vec::from_iter(io::BufReader::new(file).lines().map(|x| x.unwrap()));

                        // let strs: Vec<String> = Vec::from_iter(io::BufReader::new(file).lines().map(|x: Result<String, io::Error>| {
                        //     match x {
                        //         Ok(i) => {return i;}
                        //         Err(e) => {dbg!(e); return String::new();}
                        //     }
                        // }));
                        return Raname {
                            name: filename.to_str().unwrap().to_string(),
                            variants: strs,
                        };
                    }
                    Err(_) => {
                        return Raname::default();
                    }
                }
            }),
    );
}

impl Raname {
    fn rand_index_diap(from: usize, to: usize) -> i32 {
        rand::thread_rng().gen_range(from as i32..to as i32)
    }
    fn rand_index(to: usize) -> i32 {
        rand::thread_rng().gen_range(0..to as i32)
    }

    fn rand_name(&self) -> String {
        self.variants[Raname::rand_index(self.variants.len()) as usize].to_string()
    }

    fn left_random(&self) -> String {
        let name = self.rand_name();
        let right = Raname::rand_index(name.len() - 1usize) as usize;
        return name.chars().take(right).collect();
    }

    fn center_random(&self) -> String {
        let name = self.rand_name();
        let left = Raname::rand_index_diap(0usize, name.len() - 1usize) as usize;
        let right = Raname::rand_index_diap(left as usize, name.len()) as usize;
        return name.chars().skip(left).take(right - left).collect();
    }

    fn right_random(&self) -> String {
        let name = self.rand_name();
        let left = Raname::rand_index(name.len() - 1usize) as usize;
        return name.chars().skip(left).collect();
    }

    fn level1(&self) -> String {
        let mut prefix = "-1-> ".to_string();
        prefix.push_str(&self.rand_name());
        return prefix;
    }

    fn level2(&self) -> String {
        let mut prefix = "-2-> ".to_string();
        prefix.push_str(&self.left_random());
        prefix.push_str(&self.right_random());
        return prefix;
    }

    fn level3(&self) -> String {
        let mut prefix = "-3-> ".to_string();
        prefix.push_str(&self.left_random());
        prefix.push_str(&self.center_random());
        prefix.push_str(&self.right_random());
        return prefix;
    }

    fn level_i(&self, i: i32) -> String {
        if i == 1 {
            return self.level1();
        }
        if i == 2 {
            return self.level2();
        }
        if i == 3 {
            return self.level3();
        }
        let mut prefix = format!("-{}-> ", i + 1).to_string();
        prefix.push_str(&self.left_random());
        for _ in 2..i {
            prefix.push_str(&self.center_random());
        }
        prefix.push_str(&self.right_random());
        return prefix;
    }

    pub fn get_ranamed(&self) -> String {
        return self.level_i(rand::thread_rng().gen_range(1..7));
    }
}

fn main() {
    println!("Положите .exe файл в директорию с .dat-файлами. Файлы должны состоять из названий и быть в кодировке UTF-8");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    let files = init();

    if files.len() == 0 {
        println!("Возникла ошибка!");
        std::io::stdin().read_line(&mut String::new()).unwrap();
        std::process::exit(0)
    };
    loop {
        for x in 0..files.len() {
            println!("({:?}) {:?}", x + 1, files[x].name);
        }
        let mut readstring = String::new();
        std::io::stdin().read_line(&mut readstring).unwrap();
        readstring = readstring.trim().to_string();
        match readstring.parse::<i32>() {
            Ok(i) => {
                if i >= 0 && i < files.len() as i32 {
                    for _ in 0..20 {
                        println!("{:?}", files[i as usize - 1usize].get_ranamed());
                    }
                }
            }
            Err(_) => {}
        }
    }
}
