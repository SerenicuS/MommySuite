use std::fmt;

// =========================================================
// 1. THE BRAIN (MOMMY LANG)
// Usage: Parsing, Transpiling, Compiling, Logic, Math
// =========================================================

pub enum MommyLangError {
    // Syntax & Logic
    MissingArguments,
    InvalidVariableName,
    UndeclaredVariable,
    TypeMismatch,
    SyntaxError,
    UnclosedBlock,
    UnexpectedDone,
    

    // File / System (Specific to the Compiler)
    StatusNoFile,       // Can't find the .mommy file
    WrongFileType,      // User tried to run .txt as code
    ConfigCreationError,
    ConvertLangFailed,
    TranspilingError,
    RuntimeError,
    GCCError,

    // Math (ALU)
    MathOnString,
    DivideByZero,

    //UI
    ErrorBegins,
    ErrorEnds,
}

pub enum MommyLangStatus {
    ReadingFile,
    RenameFile,
    PrepareRun,
    SaveOnly,
    ResultOk,
    ResultOkButConfused, // "Read everything but confused"
    ResultError,
}

// =========================================================
// 2. THE HOUSE (SHELL & OS)
// Usage: Creating files, Deleting, Navigation, Windows Cmds
// =========================================================

pub enum MommyShellOk {
    // File System
    FileCreated,
    FileDeleted,
    FileRead,
    FileOpened,
    DirectoryChanged,
    DirectoryReturned,
    DirectoryCreated,
    DirectoryDeleted,
    FilesListed,

    // Process / OS
    ProcessLaunched,
    Terminated,

    // Network / Windows (Merged from OkFlavor)
    NetworkInfoRevealed, // doxxme
    PingAttempted,       // callmeplease
}

pub enum MommyShellError {
    // User Input
    GeneralInvalid,
    IncompleteArgs,
    TooManyArgs,

    // File System
    FileNotFound,
    DirectoryNotFound,
    PermissionDenied,
    RootDirectoryLocked,
    CannotOpenFile,
    CannotCreateFile,
    CannotDeleteFile,
    CannotListFiles,

    // Process / OS
    SystemCrash,
    ProcessNotFound,
    LaunchFailed,

    // Windows / Network (Merged from BadFlavor)
    ExternalIPConfigCallFail,
    ExternalCommandFailed,   // Windows command failed
    ExternalConsoleBroken,   // Console output broken
}

// =========================================================
// 3. THE VOICE (UI & MENUS)
// Usage: Menus, Prompts, Greetings, "Start Coding"
// =========================================================

pub enum MommyUI {
    // Menus
    WelcomeTitle,
    WelcomeSubtitle,
    WelcomePrompt,
    ExitMessage,

    // Registration
    AskName,
    ConfirmName,

    // Coding Mode
    PrepareCoding,     // "Do you want to instruct me?"
    StartCoding,       // "Alright sweetie, start typing..."
    RefuseCoding,      // "Why did you tell me to prepare?..."
    PrepareEnv,        // "Wait let me prepare..."
    RestartCLI,

    // Chaos / Errors
    ChaosDidNotHear,
    ChaosWrongCommand,
    GenericObedience, // "Good boy, always listen..."

    //Misc
    NewLine,
    
    
}


// =========================================================
// IMPLEMENTATIONS (TEXT RESPONSES)
// =========================================================

impl fmt::Display for MommyLangError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            Self::MissingArguments => write!(f, "I told you to properly finish what you want to say."),
            Self::UndeclaredVariable => write!(f, "I told you to name your things properly."),
            Self::InvalidVariableName => write!{f, "Really? Trying to name things that I prohibit you to use?"},
            Self::TypeMismatch => write!(f, "Matching types should be easy for you, yet you act as if it is calculus."),
            Self::SyntaxError => write!(f, "You deal with this problem, I taught you enough like an adult."),
            Self::UnclosedBlock => write!(f, "Do you know how to use punctuation?"),
            Self::UnexpectedDone => write!(f, "Periods without a sentence is wrong sweetie."),

            Self::MathOnString => write!(f, "You cannot do math on words. This isn't Algebra class."),
            Self::DivideByZero => write!(f, "Zero? You want to divide by ZERO? Get out."),

            Self::StatusNoFile => write!(f, "Mommy says there is no file to read."),
            Self::WrongFileType => write!(f, "This is not what mommy wants sweetie."),
            Self::ConfigCreationError => write!(f, "There is an error in handling the file sweetie."),
            Self::ConvertLangFailed => write!(f, "Failed to convert the file sweetie."),
            Self::TranspilingError => write!(f, "Failed to transpile the file sweetie."),
            Self::RuntimeError => write!(f, "Failed to run sweetie."),
            Self::GCCError => write!(f, "Our Friend GCC failed sweeite."),

            Self::ErrorBegins => write!(f, "--- MOMMY ERROR BEGINS ---"),
            Self::ErrorEnds => write!(f, "--- MOMMY ERROR ENDS ---"),
        }
    }
}

impl fmt::Display for MommyLangStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            Self::RenameFile => write!(f, "What do you call this instruction sweetie?"),
            Self::PrepareRun => write!(f, "Would you like mommy to follow your instruction?"),
            Self::SaveOnly => write!(f, "Alright sweetie, let me read it later."),
            Self::ResultOk => write!(f, "Mommy understood everything you told me sweetie."),
            Self::ResultOkButConfused => write!(f, "Mommy read everything but I am confused sweetie."),
            Self::ResultError => write!(f, "Mommy does not understand everything."),
            Self::ReadingFile => write!(f, "Mommy is reading the file sweetie..."),
        }
    }
}

impl fmt::Display for MommyShellOk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            Self::FileCreated => write!(f, "Here sweetie, please take care of it."),
            Self::FileDeleted => write!(f, "You don't like this? Fine, I will have it."),
            Self::FileRead => write!(f, "Do you like the contents of the file sweetie?"),
            Self::FileOpened => write!(f, "Write what is important for you, sweet boy."),
            Self::FilesListed => write!(f, "You don't trust your mommy?..."),

            Self::DirectoryChanged => write!(f, "We are here now, do you like it?"),
            Self::DirectoryReturned => write!(f, "Be careful sweetie."),
            Self::DirectoryCreated => write!(f, "Oh, you want to play house with me sweetie?"),
            Self::DirectoryDeleted => write!(f, "You don't have to do that, we can just create more house."),

            Self::ProcessLaunched => write!(f, "Are you satisfied sweetie?"),
            Self::Terminated => write!(f, "Talk to you later sweetie."),

            // The Network stuff is now here!
            Self::NetworkInfoRevealed => write!(f, "Do not tell others about our location sweetie, you only need to rely on me."),
            Self::PingAttempted => write!(f, "Are you calling someone sweetie? You do know that we only rely on each other."),
        }
    }
}

impl fmt::Display for MommyShellError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            Self::GeneralInvalid => write!(f, "Tell me the instructions correctly sweetie."),
            Self::IncompleteArgs => write!(f, "You didn't complete your sentence sweetie, are you flustered?"),
            Self::TooManyArgs => write!(f, "Greedy aren't you?"),

            Self::FileNotFound => write!(f, "You are not allowed to do that sweetie."),
            Self::DirectoryNotFound => write!(f, "I cannot find the house sweetie."),
            Self::PermissionDenied => write!(f, "This is as far as we can go sweetie."),
            Self::RootDirectoryLocked => write!(f, "Hmmm, no one is here, only your mommy right?"),
            Self::CannotOpenFile => write!(f, "Mommy doesn't know how to open this file type, or did you type it incomplete?"),
            Self::CannotCreateFile => write!(f, "Mommy cannot create the file."),
            Self::CannotDeleteFile => write!(f, "Mommy cannot delete the file."),
            Self:: CannotListFiles => write!(f, "There is nothing to list here sweetie."),

            Self::SystemCrash => write!(f, "Oh my, the system crashed."),
            Self::ProcessNotFound => write!(f, "What kind of action do you want me to do sweetie? Say it properly."),
            Self::LaunchFailed => write!(f, "I can't do it properly if you won't say clearly what you desire sweetie."),

            // The Windows Error stuff is now here!
            Self::ExternalIPConfigCallFail => write!(f, "{}", "Your friend did not respond to your calls?"),
            Self::ExternalCommandFailed => write!(f, "Your friend did not like how you commanded him. You want to make him obey?"),
            Self::ExternalConsoleBroken => write!(f, "Your friend cannot write because I broke his fingers, sorry sweetie."),

        }
    }
}

impl fmt::Display for MommyUI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            Self::WelcomeTitle => write!(f, "Hello To my Custom Shell!"),
            Self::WelcomeSubtitle => write!(f, "It is made by \"HiveMind\" to showcase my talents ^^."),
            Self::WelcomePrompt => write!(f, "Press Y or any key(to exit) key to start using it."),
            Self::ExitMessage => write!(f, "Exiting..."),

            Self::AskName => write!(f, "Do you know your name?"),
            Self::ConfirmName => write!(f, "Tell me your name sweetie.. "),
            Self::GenericObedience => write!(f, "Good boy, always listen to your mommy."),

            Self::ChaosDidNotHear => write!(f, "Are you talking sweetie? I did not hear you. Can you repeat that again?"),
            Self::ChaosWrongCommand => write!(f, "You already told me that, you are so impatient sweetie."),

            Self::PrepareCoding => write!(f, "Do you want to instruct me sweetie?"),
            Self::RefuseCoding => write!(f, "Why did you tell me to prepare it sweetie? You are wasting my time."),
            Self::PrepareEnv => write!(f, "Wait sweetie, let me prepare the environment first."),
            Self::StartCoding => write!(f, "Alright sweetie, start typing. Type 'SAVE' when you are done."),
            Self::RestartCLI => write!(f, "Okay sweetie, let us try again."),


            Self::NewLine => write!(f, "\n\n\n\n\n"),
        }
    }
}