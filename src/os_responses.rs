use std::fmt;

pub enum MommySuiteResponse {
    AppNotInit,
    AppAlreadyInit,
    RootNotFound,
    
}

pub enum MommySuiteCoreResponse {
    ShellMissing,
}

impl fmt::Display for MommySuiteResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            MommySuiteResponse::AppNotInit => write!(f, "AppContext not initialized!"),
            MommySuiteResponse::AppAlreadyInit => write!(f, "AppContext already initialized!"),
            MommySuiteResponse::RootNotFound => write!(f, "Root not found!"),
        }
    }
}


impl fmt::Display for MommySuiteCoreResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            MommySuiteCoreResponse::ShellMissing => write!(f, "CRITICAL SYSTEM FAILURE: mommy_shell missing or corrupted."),
        }
    }
}