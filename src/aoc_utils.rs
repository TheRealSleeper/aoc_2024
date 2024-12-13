use std::env::args;
use std::process::exit;

pub struct Args {
    pub part1: bool,
    pub part2: bool,
    pub verbose: bool,
    pub path: Option<String>,
    pub sample: bool,
}

const HELP: &str = "TODO: Add help text";

impl Args {
    pub fn get() -> Args {
        let mut my_args = Args {
            part1: false,
            part2: false,
            verbose: false,
            path: None,
            sample: false,
        };

        let mut env_args = args().skip(1);

        while let Some(a) = env_args.next() {
            if a.chars().nth(0).unwrap() == '-' {
                if a.chars().nth(1).unwrap() == '-' {
                    match a.chars().skip(2).collect::<String>().as_str() {
                        "part1" => my_args.part1 = true,
                        "part2" => my_args.part2 = true,
                        "verbose" => my_args.verbose = true,
                        "input" => {
                            my_args.path =
                                Some(env_args.next().expect("Input argument missing").to_string())
                        }
                        "sample" => my_args.sample = true,
                        "help" => println!("{}", HELP),
                        _ => {
                            println!("Unrecognized option {}", a);
                            exit(1);
                        }
                    }
                } else {
                    for c in a.chars().skip(1) {
                        match c {
                            '1' => my_args.part1 = true,
                            '2' => my_args.part2 = true,
                            'v' => my_args.verbose = true,
                            'i' => {
                                my_args.path = Some(
                                    env_args.next().expect("Input argument missing").to_string(),
                                )
                            }
                            's' => my_args.sample = true,
                            'h' => println!("{}", HELP),
                            _ => {
                                println!("Unrecognized option {}", a);
                                exit(1);
                            }
                        }
                    }
                }
            } else {
                println!("Invalid option '{}' given", a);
                exit(1);
            }
        }

        my_args
    }
}
