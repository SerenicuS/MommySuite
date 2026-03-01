
pub struct RequiredEXE {
    pub description: &'static str,
    pub build_path: &'static str,
    pub run_path: &'static str,
}

pub const CORE_EXE: &[RequiredEXE] = &[
    RequiredEXE {
        description: "mommy_shell.exe (communication terminal)",
        build_path: "target/debug/mommy_shell.exe",
        run_path: "mommy_bin/mommy_shell.exe"
    },
    RequiredEXE {
        description: "mommy_lang.exe (cognitive core)",
        build_path: "target/debug/mommy_lang.exe",
        run_path: "mommy_bin/mommy_lang.exe"
    },
    RequiredEXE {
        description: "mommy_editor.exe (instruction interface)",
        build_path: "mommy_editor/mommy_editor.exe",
        run_path: "mommy_bin/mommy_editor.exe"
    },
];

#[derive(Debug, Clone, Copy)]
pub enum RequiredDirectory {
    MommyBrain,
    MommyTrash,
    MommyProperties,
    Sandbox,
    Bin,
}


impl RequiredDirectory {
    pub const ALL: &'static [RequiredDirectory] = &[
        RequiredDirectory::MommyBrain,
        RequiredDirectory::MommyTrash,
        RequiredDirectory::MommyProperties,
        RequiredDirectory::Sandbox,
        RequiredDirectory::Bin,
    ];
    
    pub fn hex_code(&self) -> &'static str {
        match self {
            Self::MommyBrain => "[0x09B0]",
            Self::MommyTrash => "[0x09B4]",
            Self::MommyProperties => "[0x09C2]",
            Self::Sandbox => "[0x09CC]",
            Self::Bin => "[0x09D0]",
        }
    }
    
    pub fn dir_name(&self) -> &'static str {
        match self {
            Self::MommyBrain => "mommy_brain",
            Self::MommyTrash => "mommy_trash",
            Self::MommyProperties => "mommy_properties",
            Self::Sandbox => "sandbox",
            Self::Bin => "mommy_bin",
        }
    }
}




