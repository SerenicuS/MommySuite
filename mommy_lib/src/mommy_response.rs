use std::fmt;

pub enum MommyLangErrorResponse { //mommy_lang
    // General
    MissingArguments,
    InvalidVariableName,
    UndeclaredVariable,
    TypeMismatch,
    SyntaxError,
    UnclosedBlock,
    StatusNoFile,
    WrongFileType,
    ConfigCreationOfConfig,
    ConvertLangFailed,
    TranspilingError,
    RuntimeError,

    //ALU
    MathOnString,
    DivideByZero,
}

pub enum MommyLangGeneralResponse{
    RenameFile,
    PrepareRun,
    SaveOnly,
    StatusResultOk,
    StatusResultOkGeneral,
    StatusResultError,
    ReadingFile,


}





pub enum ShellOkResponse {
    OkGeneral,
    OkDeleteFile,
    OkCreateFile,
    OkReturnDirectory,
    OkListedFiles,
    OkMoveDirectory,
    OkTerminate,
    OkCreateDirectory,
    OkDeleteDirectory,
    OkReadFile,
    OkOpenedFile,
    OkLaunchProcess,
}

pub enum ShellErrorResponse {
    ErrorGeneral,
    ErrorBadArgs,
    ErrorTooManyArgs,
    ErrorSystem,
    ErrorFileDoesNotExist,
    ErrorPermissionDenied,
    ErrorRootDirectory,
    ErrorListedFilesDoesNotExist,
    ErrorProcessDoesNotExist,
    ErrorIncompleteLaunchProcess,
    ErrorDirectoryDoesNotExist,
    ErrorCannotOpenFile,
    ErrorCreateFile,
}

pub enum OkFlavorResponse {
    FlavorIpConfigAttempt,
    FlavorPingAttempt,
}

pub enum BadFlavorResponse {
    FlavorWindowsCallFail,
    FlavorWindowsCommandFail,
    FlavorWindowsConsoleFail,
}

pub enum GeneralFlavorResponse {
    FlavorMenu1,
    FlavorMenu2,
    FlavorMenu3,
    FlavorExit,
    FlavorRegister1,
    FlavorRegister2,
    FlavorStart1,
    FlavorChaosNotHear,
    FlavorChaosWrongCommand,
    FlavorPrepareCoding,
    FlavorRefuseCoding,
    FlavorPrepareEnv,
    FlavorStartCoding,
}

impl fmt::Display for MommyLangErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            Self::MissingArguments => write!(f, "{}", "I told you to properly finish what you want to say."),
            Self::UndeclaredVariable => write!(f, "{}", "I told you to name your things properly."),
            Self::InvalidVariableName => write!{f, "{}", "Really? Trying to name things that I prohibit you to use?"},
            Self::TypeMismatch => write!(f, "{}", "Matching types should be easy for you, yet you act as if it is a calculus"),
            Self::SyntaxError => write!(f, "{}", "You deal with this problem, I taught you enough like an adult"),
            Self::UnclosedBlock => write!(f, "{}", "Do you know how to use punctuations?"),

            Self::MathOnString => write!(f, "You cannot do math on words. This isn't Algebra class."),
            Self::DivideByZero => write!(f, "Zero? You want to divide by ZERO? Get out."),


            Self::StatusNoFile => write!(f, "{}", "Mommy says there is no file to read."),
            Self::WrongFileType => write!(f, "{}", "This is not what mommy wants sweetie."),
            Self::ConfigCreationOfConfig => write!(f, "{}", "There is an error in handling the file sweetie"),
            Self::ConvertLangFailed => write!(f, "{}", "Failed to convert the file sweetie."),
            Self::TranspilingError => write!(f, "{}", "Failed to transpile the file sweetie"),
            Self::RuntimeError => write!(f, "{}", "Failed to run sweetie"),
        }
    }
}

impl fmt::Display for MommyLangGeneralResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            Self::RenameFile => write!(f, "{}", "What do you call this instruction sweetie?"),
            Self::PrepareRun => write!(f, "{}", "Would you like mommy to follow your instruction?"),
            Self::SaveOnly => write!(f, "{}", "Alright sweetie, let me read it later."),
            Self::StatusResultOk => write!(f, "{}", "Mommy understood everything you told me sweetie."),
            Self::StatusResultOkGeneral => write!(f, "{}", "Mommy read everything but I am confused sweetie."),
            Self::StatusResultError => write!(f, "{}", "Mommy does not understand everything."),
            Self::ReadingFile => write!(f, "{}", "Mommy is reading the file sweetie..."),
        }
    }
}


impl fmt::Display for OkFlavorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            Self::FlavorIpConfigAttempt => write!(f, "{}", "Do not tell others about our location sweetie, you only need to rely on me."),
            Self::FlavorPingAttempt => write!(f, "{}", "Are you calling someone sweetie? You do know that we only rely on each other."),


        }
    }
}

impl fmt::Display for BadFlavorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            Self::FlavorWindowsCallFail => write!(f, "{}", "Your friend did not respond to your calls?"),
            Self::FlavorWindowsCommandFail => write!(f, "{}", "Your friend did not like how you commanded him. You want to make him obey?"),
            Self::FlavorWindowsConsoleFail => write!(f, "{}", "Your friend cannot write because i broke his fingers, sorry sweetie."),
        }
    }
}

impl fmt::Display for ShellErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            Self::ErrorGeneral => write!(f, "Tell me the instructions correctly sweetie."),
            Self::ErrorBadArgs => write!(f, "You didn't complete your sentence sweetie, are you flustered?."),
            Self::ErrorTooManyArgs => write!(f, "Greedy Aren't you?."),
            Self::ErrorSystem => write!(f, "Oh my, the system crashed."),
            Self::ErrorFileDoesNotExist => write!(f, "You are not allowed to do that sweetie?"),
            Self::ErrorPermissionDenied => write!(f, "This is as far as we can go sweetie."),
            Self::ErrorRootDirectory => write!(f, "Hmmm, no one is here, only your mommy right?."),
            Self::ErrorListedFilesDoesNotExist => write!(f, "Hmmm, no one is here, only your mommy right?."),
            Self::ErrorProcessDoesNotExist => write!(f, "What kind of action you want me to do sweetie? Say it properly."),
            Self::ErrorIncompleteLaunchProcess => write!(f, "I can't do it properly if you won't say clearly what you desire sweetie."),
            Self::ErrorDirectoryDoesNotExist => write!(f, "I cannot find the house sweetie."),
            Self::ErrorCannotOpenFile => write!(f, "Mommy doesn't know how to open this file type, or did you type it incomplete?"),
            Self::ErrorCreateFile => write!(f, "Mommy cannot create the file."),
        }
    }
}


impl fmt::Display for GeneralFlavorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            Self::FlavorMenu1 => write!(f, "Hello To my Custom Shell!"),
            Self::FlavorMenu2 => write!(f, "It is made by \"HiveMind\" to showcase my talents ^^."),
            Self::FlavorMenu3 => write!(f, "Press Y or any key(to exit) key to start using it. "),
            Self::FlavorExit => write!(f, "Exiting....."),
            Self::FlavorRegister1 => write!(f, "Do you know your name?"),
            Self::FlavorRegister2 => write!(f, "Tell me your name sweetie.. "),
            Self::FlavorStart1 => write!(f, "Good boy, always listen to your mommy."),
            Self::FlavorChaosNotHear => write!(f, "Are you talking sweetie? I did not hear you. Can you repeat that again?"),
            Self::FlavorChaosWrongCommand => write!(f, "You already told me that, you are so impatient sweetie."),
            Self::FlavorPrepareCoding => write!(f, "Do you want to instruct me sweetie?"),
            Self::FlavorRefuseCoding => write!(f, "Why did you told me to prepare it sweetie? You are wasting my time."),
            Self::FlavorPrepareEnv => write!(f, "Wait sweetie, let me prepare the environment first."),
            Self::FlavorStartCoding => write!(f, "Alright sweetie, start typing. Type 'SAVE' when you are done."),

        }
    }
}

impl fmt::Display for ShellOkResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{

            Self::OkGeneral => write!(f, "It was successful sweetie."),
            Self::OkDeleteFile => write!(f, "You don't like this? Fine, I will have it."),
            Self::OkCreateFile => write!(f, "Here sweetie, please take care of it."),
            Self::OkReturnDirectory => write!(f, "Be careful sweetie."),
            Self::OkListedFiles => write!(f, "You don't trust your mommy?..."),
            Self::OkMoveDirectory => write!(f, "We are here now, do you like it?"),
            Self::OkTerminate => write!(f, "Talk to you later sweetie."),
            Self::OkCreateDirectory => write!(f, "Oh, you want to play house with me sweetie?"),
            Self::OkDeleteDirectory => write!(f, "You don't have to do that, we can just create more house."),
            Self::OkReadFile => write!(f, "Do you like the contents of the file sweetie?"),
            Self::OkOpenedFile => write!(f, "Write what is important for you, sweet boy."),
            Self::OkLaunchProcess => write!(f, "Are you satisfied sweetie?."),

        }
    }
}

