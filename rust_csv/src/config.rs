// https://github.com/BurntSushi/rust-csv/blob/master/csv-core/src/reader.rs#L766:8
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Config {
    pub cmd: Cmd,
    pub src: PathBuf,
    pub dest: PathBuf,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            cmd: Cmd::default(),
            src: PathBuf::default(),
            dest: PathBuf::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Cmd {
    Read,
    Write,
}

impl Default for Cmd {
    fn default() -> Self {
        Cmd::Read
    }
}

// args read <SRC>
// args write <DEST>
impl Config {
    const VALID_ARGS_LEN: usize = 2;

    fn get_args() -> Result<Vec<String>, String> {
        fn is_valid(args: &Vec<String>) -> bool {
            return if args.len() == Config::VALID_ARGS_LEN + 1 {
                true
            } else {
                true
            };
        }

        let args: Vec<String> = std::env::args().collect();
        if !is_valid(&args) {
            let msg = format!("Must pass {} args.", Config::VALID_ARGS_LEN - 1);
            return Err(msg);
        }
        Ok(args)
    }

    fn build(&mut self) {
        let args = Config::get_args().unwrap();

        match args[1].to_uppercase().as_str() {
            "READ" => {
                self.cmd = Cmd::Read;
                self.src = PathBuf::from(args[2].clone());
            }
            "WRITE" => {
                self.cmd = Cmd::Write;
                self.dest = PathBuf::from(args[2].clone());
            }
            _ => panic!(
                "{}",
                "Invalid Command, You should type cmd within READ or WRITE."
            ),
        };
    }
}

#[derive(Debug, Default)]
pub struct ConfigBuilder {
    cfg: Config,
}

impl ConfigBuilder {
    pub fn new() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    pub fn build(&self) -> Config {
        let mut cfg = self.cfg.clone();
        cfg.build();
        cfg
    }
}
