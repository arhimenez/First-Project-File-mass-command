use std::env;
use std::path::PathBuf;
use std::fs;
use std::fs::read_dir;
use std::io::ErrorKind;
use std::process;
use std::path::Path;

#[derive(Debug)]
struct Arguments {
    flag: String,
    f_type: String,
    f_type_get: String,
    path: PathBuf,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() > 5 {
            return Err("Too few arguments were given, help:");
        } else if args.len() < 3 {
            return Err("not enough arguments, help:");
        }
        let p = PathBuf::from(&args[args.len()-1]);
        //println!("{}", &p.display());
        if let Ok(mut bool) = p.try_exists() {
            if p.is_dir() != true {
                bool = false;
            }
            //println!("{bool}");
            match bool {
                true => {
                    let f = args[1].clone();
                    let t1 = args[2].clone();
                    //println!("{}", args.len());
                    if args.len() == 5 {
                        return Ok(Arguments { flag: f, f_type: t1, f_type_get: args[3].clone(), path: p });
                    } else {
                        return Ok(Arguments {flag: f, f_type: t1, f_type_get: String::from(""), path: p })
                    }
                },
                false => return Err("Broken path/Points to a specific file"),
            }
        } else {
            return Err("Weirdly permission denied error");
        }
    }
}

fn cat (arguments: Arguments) -> () {
    if let Ok(path_iter) = read_dir(&arguments.path) {
        for path in path_iter {
            let mut path_buff = path.unwrap().path();
            if let Some(extension_str) = path_buff.extension().unwrap().to_str() {
                if extension_str == arguments.f_type {
                    let mut tmp_buff = path_buff.clone();
                    tmp_buff.set_extension(&arguments.f_type_get);

                    if let Ok(_) = fs::rename(&path_buff, &tmp_buff) {
                        println!("Changed: {}\n--> {}", path_buff.display(), tmp_buff.display());
                    } else {
                        panic!("COULDN'T RENAME:\n {}\n THUS STOPPING EXEC", path_buff.display());
                    }
                }
            }
        }
    }
    println!("Supposedly worked, didn't catch errors");
}

fn dat(arguments: Arguments) -> () {
    if let Ok(path_iter) = read_dir(&arguments.path) {
        for path in path_iter {
            let path_buff = path.unwrap().path();
            if let Some(extension_str) = path_buff.extension().unwrap().to_str() {
                if extension_str == arguments.f_type {
                    fs::remove_file(&path_buff).unwrap();
                }
            }
        }
    } else {
        panic!("Couldn't create an iterator wtf");
    }
    println!("Supposedly worked, didn't catch errors");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    //println!("{:?}", args);
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                println!("flag: -dat is for delete all types, example of use:\n\
                -dat mp4 C:\\Users\\Avinoam\\PycharmProjects\\finalPythonHw\nflag: -cat is for change
                all types, example of use:\n
                -cat mp4 mp3 C:\\Users\\Avinoam\\PycharmProjects\\finalPythonHw\n#########FOLDER CANT CONTAIN SPACE KEY IN ITS NAME #############");
                process::exit(0);
            } else {
                eprintln!("Program: {}\nError: {}", program, err);
                process::exit(0);
            }
        }
    );

    //println!("{:?}", arguments);
    if arguments.f_type.len() > 0 && arguments.f_type_get.len() > 0 && arguments.flag == String::from("-cat") {
        cat(arguments);
    } else if arguments.flag == String::from("-dat") {
        dat(arguments);
    } else {
        println!("invalid");
    }
}

// -dat mp4 C:\Users\Avinoam\PycharmProjects\finalPythonHw