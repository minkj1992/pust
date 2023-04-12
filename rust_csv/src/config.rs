// https://github.com/BurntSushi/rust-csv/blob/master/csv-core/src/reader.rs#L766:8
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Config {
    pub src: PathBuf,
}

// TODO: Check why should we use this impl.
impl Default for Config {
    fn default() -> Config {
        Config {
            src: PathBuf::default(),
        }
    }
}

impl Config {
    const VALID_ARGS_LEN: usize = 1;

    fn get_args() -> Result<Vec<String>, String> {
        fn is_valid(args: &Vec<String>) -> bool {
            return if args.len() == Config::VALID_ARGS_LEN + 1 {
                true
            } else {
                false
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
        let args = match Config::get_args() {
            Ok(a) => a,
            Err(e) => panic!("{}", e),
        };
        self.src = PathBuf::from(args[1].clone());
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
