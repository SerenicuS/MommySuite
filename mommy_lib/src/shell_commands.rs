
pub enum MommyShellCommands {
    ShellHelp,                          // tellme
    ShellHelpAdvanced,                  // tellmesecret
    ShellExit,                          // mayileave
    ShellCurrentDirectory,              // iamhere
    ShellListFilesCurrentDirectory,     // mommy?
    ShellChangeDirectory,               // walkwithme <file_name>
    ShellReturnToPrevDirectory,         // goback
    ShellCreateFile,                    // canihave <filename>
    ShellDeleteFile,                    // takethe <file_name>
    ShellOpenFile,                      // openthis <file_name>
    ShellReadFile,                      // readthis <file_name>
    ShellShowIPConfig,                  // doxxme
    ShellPing,                          // callmeplease <ip/dns>
    ShellRunFile,                       // runthis <file_name>
    ShellStartCoding,                   // startcoding
    ShellUnknownCommand,                // unknown command
    ShellClear,                         // clear
    ShellCreateDir,                     // letusplayhouse    
    ShellDeleteDir,                     // removethehouse
    ShellChangeCodeDir                  // changeoutput

}


impl MommyShellCommands {
    pub fn from_str(token: &str) -> Self {
        match token {
            "tellme" => MommyShellCommands::ShellHelp,
            "tellmesecret" => MommyShellCommands::ShellHelpAdvanced,
            "mayileave" => MommyShellCommands::ShellExit,
            "iamhere" => MommyShellCommands::ShellCurrentDirectory,
            "mommy?" => MommyShellCommands::ShellListFilesCurrentDirectory,
            "walkwithme" => MommyShellCommands::ShellChangeDirectory,
            "goback" => MommyShellCommands::ShellReturnToPrevDirectory,
            "canihave" => MommyShellCommands::ShellCreateFile,
            "takethe" => MommyShellCommands::ShellDeleteFile,
            "openthis" => MommyShellCommands::ShellOpenFile,
            "readthis" => MommyShellCommands::ShellReadFile,
            "doxxme" => MommyShellCommands::ShellShowIPConfig,
            "callmeplease" => MommyShellCommands::ShellPing,
            "runthis" => MommyShellCommands::ShellRunFile,
            "startcoding" => MommyShellCommands::ShellStartCoding,
            "clear" => MommyShellCommands::ShellClear,
            "letusplayhouse" => MommyShellCommands::ShellCreateDir,
            "removethehouse" => MommyShellCommands::ShellDeleteDir,
            "changeoutput" => MommyShellCommands::ShellChangeCodeDir,
            _ => MommyShellCommands::ShellUnknownCommand,
        }
    }
}
