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
    VariableAlreadyExists,

    // File / System (Specific to the Compiler)
    StatusNoFile,       // Can't find the .mommy file
    WrongFileType,      // User tried to run .txt as code
    ConfigCreationError,
    ConvertLangFailed,
    TranspilingError,
    RuntimeError,
    GCCError,
    CannotReadFile,

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
    CheckingFile,
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

    // Network / Windows
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
    RootDirError,

    // Process / OS
    SystemCrash,
    ProcessNotFound,
    LaunchFailed,

    // Windows / Network
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
            // LOGIC ERRORS (Brutalized)
            Self::MissingArguments => write!(f, "You stopped talking mid-sentence. Finish what you started, or don't speak at all."),
            Self::UndeclaredVariable => write!(f, "Who is that? You are making up names again. Define them first."),
            Self::InvalidVariableName => write!(f, "That name is forbidden. Do not test my authority."),
            Self::TypeMismatch => write!(f, "You are trying to fit a square peg in a round hole. Are you doing this on purpose to annoy me?"),
            Self::SyntaxError => write!(f, "I can't even read this mess. Fix your grammar before I lose my patience."),
            Self::UnclosedBlock => write!(f, "You opened a door and forgot to close it. Were you raised in a barn? Close your blocks."),
            Self::UnexpectedDone => write!(f, "You said 'done' but you haven't even started anything. Focus."),
            Self::VariableAlreadyExists => write!(f, "We already have a variable named that. Be creative, or be silent."),

            // MATH ERRORS
            Self::MathOnString => write!(f, "You cannot do math on words. Stop acting childish."),
            Self::DivideByZero => write!(f, "Divide by zero? Do you WANT to break the universe? Don't be stupid."),

            // SYSTEM ERRORS
            Self::StatusNoFile => write!(f, "There is nothing here. Stop wasting my time."),
            Self::WrongFileType => write!(f, "I don't read trash. Give me a .mommy file."),
            Self::ConfigCreationError => write!(f, "I couldn't even prepare the file. You broke something deep, didn't you?"),
            Self::ConvertLangFailed => write!(f, "I refuse to convert this garbage into C code."),
            Self::TranspilingError => write!(f, "The translation failed. Your logic makes no sense."),
            Self::RuntimeError => write!(f, "It crashed. I told you it would crash."),
            Self::GCCError => write!(f, "Even GCC is refusing to work with you. Pathetic."),
            Self::CannotReadFile => write!(f, "I cannot read this file, have you ever read it?"),

            Self::ErrorBegins => write!(f, "--- MOMMY IS DISAPPOINTED ---"),
            Self::ErrorEnds => write!(f, "--- END OF FAILURE ---"),
        }
    }
}

impl fmt::Display for MommyLangStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            Self::RenameFile => write!(f, "Name your creation. Make it good."),
            Self::PrepareRun => write!(f, "Shall I execute this? nod if you are sure."),
            Self::SaveOnly => write!(f, "Fine. I'll keep it, but I won't run it."),
            Self::ResultOk => write!(f, "Good boy. You actually made sense this time."),
            Self::ResultOkButConfused => write!(f, "I did what you asked, but your logic is... questionable."),
            Self::ResultError => write!(f, "No. I am not doing that. Look at your errors."),
            Self::ReadingFile => write!(f, "Shh. Mommy is reading..."),
            Self::CheckingFile => write!(f, "Mommy is reading every line... If I find a virus, malware, or a script trying to help you leave me, I will delete it. And then I will punish you.....")
        }
    }
}

impl fmt::Display for MommyShellOk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            Self::FileCreated => write!(f, "I created it for you. You better take care of it."),
            Self::FileDeleted => write!(f, "Gone. I hope you didn't need that."),
            Self::FileRead => write!(f, "Here is what you asked for. Happy now?"),
            Self::FileOpened => write!(f, "The file is open. Don't write nonsense."),
            Self::FilesListed => write!(f, "This is everything I allow you to see."),

            Self::DirectoryChanged => write!(f, "We are here now. Don't wander off."),
            Self::DirectoryReturned => write!(f, "Back to safety. Good."),
            Self::DirectoryCreated => write!(f, "A new room for us. Keep it clean."),
            Self::DirectoryDeleted => write!(f, "I removed that place. It was cluttering my house."),

            Self::ProcessLaunched => write!(f, "I let it run. Watch it closely."),
            Self::Terminated => write!(f, "I killed it. Silence is better."),

            Self::NetworkInfoRevealed => write!(f, "This is where we live. Don't tell strangers."),
            Self::PingAttempted => write!(f, "Calling out? You know you only need to rely on me, right?"),
        }
    }
}

impl fmt::Display for MommyShellError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            Self::GeneralInvalid => write!(f, "Speak clearly. I don't tolerate mumbling."),
            Self::IncompleteArgs => write!(f, "You didn't finish your command. Are you nervous?"),
            Self::TooManyArgs => write!(f, "One thing at a time. Don't be greedy."),

            Self::FileNotFound => write!(f, "It's not there. Are you hallucinating?"),
            Self::DirectoryNotFound => write!(f, "That place doesn't exist. Stop making things up."),
            Self::PermissionDenied => write!(f, "Don't touch that. That is MINE."),
            Self::RootDirectoryLocked => write!(f, "You are trying to leave me? The door is locked."),
            Self::CannotOpenFile => write!(f, "I can't open that. It's either broken or you're incompetent."),
            Self::CannotCreateFile => write!(f, "I refuse to create that file."),
            Self::CannotDeleteFile => write!(f, "I'm keeping that file. You don't get to delete it."),
            Self::CannotListFiles => write!(f, "There is nothing here for you."),
            Self::RootDirError => write!(f, "Mommy can't find the floor."),

            Self::SystemCrash => write!(f, "Look what you did. You broke it. Now I have to clean up your mess."),
            Self::ProcessNotFound => write!(f, "That doesn't exist. Focus, sweetie."),
            Self::LaunchFailed => write!(f, "It refused to start. Probably because you asked nicely instead of demanding it."),

            // Windows / Network
            Self::ExternalIPConfigCallFail => write!(f, "Your little friend isn't answering. Just you and me now."),
            Self::ExternalCommandFailed => write!(f, "That command failed. Try harder."),
            Self::ExternalConsoleBroken => write!(f, "I broke his fingers. He won't be writing back."),
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

            Self::AskName => write!(f, "Do you know who you are?"),
            Self::ConfirmName => write!(f, "Tell me your name. Don't lie."),
            Self::GenericObedience => write!(f, "Good boy. Always listen to your mommy."),

            Self::ChaosDidNotHear => write!(f, "I didn't hear you. Speak up."),
            Self::ChaosWrongCommand => write!(f, "You are repeating yourself. I hate repetition."),

            Self::PrepareCoding => write!(f, "Do you have instructions for me? Make them count."),
            Self::RefuseCoding => write!(f, "You called me over and then said nothing? Do not waste my time."),
            Self::PrepareEnv => write!(f, "Wait. I need to prepare the room."),
            Self::StartCoding => write!(f, "Start typing. Type 'SAVE' when you are done. Don't bore me."),
            Self::RestartCLI => write!(f, "Let's try that again. Do it right this time."),

            Self::NewLine => write!(f, "\n\n\n"),
        }
    }
}