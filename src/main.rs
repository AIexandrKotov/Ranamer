use std::fs::File;
use std::io::{self, BufRead};
use std::iter::FromIterator;
use std::path::{PathBuf, Path};
extern crate rand;
extern crate walkdir;
use rand::Rng;
use walkdir::{DirEntry, WalkDir};
use argh::FromArgs;

#[derive(Default)]
struct Raname {
    name: String,
    variants: Vec<(String, usize)>,
}

// fn is_hidden(entry: &DirEntry) -> bool {
//     entry.file_name()
//          .to_str()
//          .map(|s| s.starts_with("."))
//          .unwrap_or(false)
// }

fn init(pth: &str, extensions: Vec<&str>) -> Vec<Raname> {
    //let pth = std::fs::canonicalize(PathBuf::from("./")).unwrap();
    let wlkdir = WalkDir::new(Path::parent(&PathBuf::from(pth)).unwrap());
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
                    .and_then(|y| if extensions.contains(&y.to_str().unwrap()) { Some(y) } else { None })
                    .is_some()
            })
            .map(|x| {
                match x {
                    Ok(path) => {
                        let filename = path.file_name();
                        let file = File::open(path.path()).unwrap();
                        let strs = Vec::from_iter(io::BufReader::new(file).lines().map(|x| {
                            let size = x.as_ref().unwrap().chars().count();
                            (x.unwrap(), size)
                        }));

                        return Raname {
                            name: filename.to_str().unwrap().to_owned(),
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
    fn rand_index_diap(from: usize, to: usize) -> usize {
        rand::thread_rng().gen_range(from..to)
    }
    fn rand_index(to: usize) -> usize {
        rand::thread_rng().gen_range(0..to)
    }

    fn rand_name(&self) -> (&str, usize) {
        let res = &self.variants[Raname::rand_index(self.variants.len())];
        (res.0.as_str(), res.1)
    }

    fn left_random(&self) -> &str {
        let name = self.rand_name();
        let right = Raname::rand_index(name.1 - 1usize);
        return &name.0[..name.0.char_indices().take(right + 1).last().unwrap().0];
    }

    fn center_random(&self) -> &str {
        let name = self.rand_name();
        let left = Raname::rand_index_diap(0usize, name.1 - 1usize);
        let right = Raname::rand_index_diap(left, name.1);
        return &name.0[name.0.char_indices().skip(left).next().unwrap().0
            ..name
                .0
                .char_indices()
                .skip(left)
                .take(right - left + 1)
                .last()
                .unwrap()
                .0];
    }

    fn right_random(&self) -> &str {
        let name = self.rand_name();
        let left = Raname::rand_index(name.1 - 1usize) as usize;
        return &name.0[name.0.char_indices().skip(left).next().unwrap().0..];
    }

    fn level1(&self) -> String {
        let mut prefix = "-1-> ".to_string();
        prefix.push_str(&self.rand_name().0);
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

    pub fn get_ranamed(&self, depth: i32) -> String {
        return self.level_i(rand::thread_rng().gen_range(1..depth));
    }
}

fn fake_readline() -> () {
    std::io::stdin().read_line(&mut String::new()).unwrap();
}

#[derive(FromArgs)]
/// Raname
struct Raptions {
    /// generation count
    #[argh(option, default = "20", short = 'g')]
    generation_count: i32,
    /// available extensions
    #[argh(option, default = "String::from(\"dat,txt\")", short = 'e')]
    available_extensions: String,
    #[argh(option, default = "7", short = 'd')]
    /// max merging words
    depth: i32,
}

fn main() {
    let ah = argh::from_env::<Raptions>();
    let args: Vec<String> = std::env::args().collect();
    let direct_launch = if args.len() == 1usize {true} else {false};    
    if direct_launch {
        println!("Положите .exe файл в директорию с .dat-файлами. Файлы должны состоять из названий и быть в кодировке UTF-8");
        fake_readline();
    } 

    let files = init(&args[0], Vec::from_iter(ah.available_extensions.split(',')));

    

    if files.len() == 0 {
        println!("Программа не обнаружила необходимые файлы");
        if direct_launch {
            fake_readline();
        } 
        std::process::exit(0)
    }
    else
    {
        let instant = std::time::Instant::now();
        for _ in 0..100_000 {
            files[0].get_ranamed(ah.depth);
        }
        println!("Test 100k generations: {:?}", instant.elapsed());
    }
    loop {
        for x in 0..files.len() {
            println!("({}) {}", x + 1, files[x].name);
        }
        let mut readstring = String::new();
        std::io::stdin().read_line(&mut readstring).unwrap();
        readstring = readstring.trim().to_string();
        match readstring.parse::<i32>() {
            Ok(i) => {
                if i >= 0 && i < files.len() as i32 {
                    for _ in 0..ah.generation_count {
                        println!("{}", files[i as usize - 1usize].get_ranamed(ah.depth));
                    }
                }
            }
            Err(_) => {
                println!("Неверный ввод.");
            }
        }
    }
}
