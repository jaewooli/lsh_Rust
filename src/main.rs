use std::io::{Read, BufRead, stdin};
use std::env::set_current_dir;
use std::process::{exit, Command};
use std::thread::spawn;

type BuilitinFunc = fn(args: Vec<String>) -> i32;

static BUILITIN_STR: [&str; 3] = ["cd", "help", "exit"];
static BUILITIN_FUNC: &[BuilitinFunc; 3] = &[lsh_cd, lsh_help, lsh_exit];

static LSH_USE_STD_GETLINE: bool = true;
static LSH_RL_BUFSIZE: i32 = 1024;
static LSH_TOK_BUFSIZE: i32 = 64;
static LSH_TOK_DELIM: [char; 4] = [' ', '\t', '\n', '\r'];

fn lsh_cd(args: Vec<String>) -> i32 {
    if args.len() < 2 {
        eprintln!("lsh: expected argument to \"cd\"");
    } else {
        if let Err(err) = set_current_dir(&args[1]) {
            eprintln!("lsh: {}: {}", args[1], err);
        }
    }
    1
}

fn lsh_help(_args: Vec<String>) -> i32 {
    println!("Stephen Brennan's LSH");
    println!("Type program names and arguments, and hit enter.");
    println!("The following are built in:");

    for func in BUILITIN_STR.iter() {
        println!("{}", func);
    }

    println!("Use the man command for information on other programs.");
    1
}

fn lsh_exit(_args: Vec<String>) -> i32 {
    0
}

fn lsh_launch(args: Vec<String>) -> i32 {
    let handle = spawn(move || {
        let status = Command::new(args[0].clone()).args(&args[1..]).status();

        match status {
            Ok(exit_status) => {
                if exit_status.success() {
                    println!("Shell common Command executed successfully");
                } else {
                    eprintln!(
                        "Command failed with exit code: {}",
                        exit_status.code().unwrap()
                    );
                }
            }
            Err(err) => {
                eprintln!("lsh: {}", err);
            }
        }
    });

    handle.join().unwrap();
    1
}

fn lsh_execute(args: Vec<String>) -> i32 {
    if args.len() < 1 {
        return 1;
    }
    
    for (index, func) in BUILITIN_FUNC.iter().enumerate() {
        println!("{}", args[0]);
        if args[0] == BUILITIN_STR[index] {
            println!("{}", args[0]);
            return (*func)(args);
        }
    }

    return lsh_launch(args);
}

fn lsh_read_line() -> String {
    if LSH_USE_STD_GETLINE {
        let stdin = stdin();
        let mut handle = stdin.lock();
        let mut line = String::new();

        match handle.read_line(&mut line) {
            Ok(0) => {
                exit(0);
            }
            Ok(_) => {},
            Err(error) => {
                eprintln!("lsh: getline {}", error);
                exit(1);
            }
        };
        return line
    } else {
        // LSH_RL_BUFSIZE = 1024;
        let bufsize: i32 = LSH_RL_BUFSIZE;
        let mut position: i32 = 0;
        let mut buffer = Vec::with_capacity(bufsize as usize);

        loop {
            let mut c = [0u8; 1];
            if io::stdin().read_exact(&mut c).is_err() {
                if buffer.is_empty() {
                    exit(0);
                } else {
                    break;
                }
            }

            if c[0] == b'\n' {
                break;
            }

            buffer.push(c[0]);
            position += 1;

            if position >= buffer.capacity() as i32 {
                buffer.reserve(LSH_RL_BUFSIZE as usize);
            }
        }
        String::from_utf8(buffer)
            .map_err(|_| {
                exit(1);
            }).expect("Exit")
    }
}

fn lsh_split_line(line: String) -> Vec<String> {
    let bufsize: i32 = LSH_TOK_BUFSIZE;
    let mut tokens = Vec::with_capacity(bufsize as  usize);

    let words: Vec<&str> = line
        .split(|c| LSH_TOK_DELIM.contains(&c))
        .filter(|s| !s.is_empty())
        .collect();

    for (index, word) in words.iter().enumerate() {
        tokens.push(word.to_string());
        if index >= bufsize as usize{
            tokens.reserve(bufsize as usize);
        }
    }
    tokens
}

fn lsh_loop() {
    loop {
        print!("> ");
        let line = lsh_read_line();
        println!("line: {:?}",line);
        let args = lsh_split_line(line);
        println!("args: {:?}",args);
        let status = lsh_execute(args);
        println!("status: {:?}",status);

        if status == 0 {
            break;
        }
    }
}

fn main() {
    lsh_loop();
    exit(0);
}
