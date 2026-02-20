// ============================================================================
// SHARED EDITOR UTILITIES
// ============================================================================
// This module provides common functionality for editor operations

use std::env;
use std::path::PathBuf;

/// Get the absolute path to the mommy_editor executable
/// 
/// Tries to locate the editor relative to the shell executable location.
/// Falls back to relative path if workspace structure cannot be determined.
pub fn get_editor_path() -> PathBuf {
    if let Ok(shell_exe) = env::current_exe() {
        // Shell exe is at: MommySuite/target/debug/mommy_shell.exe
        if let Some(target_dir) = shell_exe.parent() {
            // target/debug
            if let Some(target) = target_dir.parent() {
                // target
                if let Some(workspace_root) = target.parent() {
                    // MommySuite - construct absolute path to editor
                    let editor = workspace_root.join("mommy_editor").join("mommy_editor.exe");
                    return editor;
                }
            }
        }
    }
    // Fallback if we can't determine workspace root
    PathBuf::from("./mommy_editor/mommy_editor.exe")
}

