use mommy_lib::constants;
use mommy_lib::responses;

pub struct Config {
    pub input_path: String,
    pub c_path: String,
    pub exe_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < constants::ARGS_MIN_FILE {
            return Err(responses::MommyLangError::StatusNoFile.to_string());
        }

        let input_path = args[constants::IDX_FILE_NAME].clone();

        if !input_path.ends_with(constants::EXT_SOURCE) {
            return Err(responses::MommyLangError::WrongFileType.to_string());
        }

        let c_path = input_path.replace(constants::EXT_SOURCE, constants::EXT_C);
        let exe_path = input_path.replace(constants::EXT_SOURCE, constants::EXT_EXE);

        Ok(Config {
            input_path,
            c_path,
            exe_path,
        })
    }
}

