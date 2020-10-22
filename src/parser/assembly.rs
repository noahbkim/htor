use std::io::{Write};
use std::process::{Command, Stdio};
use super::error::AnonymousRuntimeError;

pub fn compile_assembly(contents: &String) -> Result<Vec<u8>, AnonymousRuntimeError>{
    let mut child = Command::new("gcc")
        .arg("-c") // Compile assembly
        .arg("-o") // Output file path
        .arg(file.path().to_str().unwrap())
        .arg("-x") // Read from STDIN
        .arg("assembler")
        .arg("-")
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|e| AnonymousRuntimeError::new(format!("failed to run gcc: {}", e)))?;

    {
        let stdin = child.stdin.as_mut().ok_or(
            AnonymousRuntimeError::new("failed to communicate with gcc process".to_string()))?;
        stdin.write_all(contents.as_bytes()).map_err(|e|
            AnonymousRuntimeError::new(format!("failed to write assembly to gcc pipe: {}", e),))?;
    }

    let output = child.wait_with_output().map_err(|e|
        AnonymousRuntimeError::new(format!("error while awaiting gcc: {}", e)))?;

    if output.status.success() {
        let file = elf::File::open_path(file.path()).map_err(|e|
            AnonymousRuntimeError::new(format!("failed to open elf file: {:?}", e)))?;
        match file.get_section(".text") {
            Some(section) => Ok(section.data.clone()),
            None => Err(AnonymousRuntimeError::new("failed to find .text in elf file".to_string())),
        }
    } else {
        Err(AnonymousRuntimeError::new("compilation of assembly failed".to_string()))
    }
}
