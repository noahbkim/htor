use super::{Block, RawMacroBlock};
use crate::error::{AnonymousEvaluationError, EvaluationError};
use crate::evaluator::scope::EvaluatorScope;
use std::io::Write;
use std::process::{Command, Stdio};
use std::rc::Rc;

fn compile_assembly(contents: &String) -> Result<Vec<u8>, AnonymousEvaluationError> {
    let file = tempfile::NamedTempFile::new().map_err(|e| {
        AnonymousEvaluationError::new(format!("error creating temporary file: {}", e))
    })?;

    let mut child = Command::new("gcc")
        .arg("-c") // Compile assembly
        .arg("-o") // Output file path
        .arg(file.path().to_str().unwrap())
        .arg("-x") // Read from STDIN
        .arg("assembler")
        .arg("-")
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|e| AnonymousEvaluationError::new(format!("failed to run gcc: {}", e)))?;

    {
        let stdin = child.stdin.as_mut().ok_or(AnonymousEvaluationError::new(
            "failed to communicate with gcc process".to_string(),
        ))?;
        stdin.write_all(contents.as_bytes()).map_err(|e| {
            AnonymousEvaluationError::new(format!("failed to write assembly to gcc pipe: {}", e))
        })?;
    }

    {
        let output = child.wait_with_output().map_err(|e| {
            AnonymousEvaluationError::new(format!("error while awaiting gcc: {}", e))
        })?;
        if !output.status.success() {
            return Err(AnonymousEvaluationError::new(
                "compilation of assembly failed".to_string(),
            ));
        }
    }

    let binary = elf::File::open_path(file.path())
        .map_err(|e| AnonymousEvaluationError::new(format!("failed to open elf file: {:?}", e)))?;
    match binary.get_section(".text") {
        Some(section) => Ok(section.data.clone()),
        None => Err(AnonymousEvaluationError::new(
            "failed to find .text in elf file".to_string(),
        )),
    }
}

pub struct AssemblyBlock {
    line_number: usize,
    compiled: Vec<u8>,
}

impl Block for AssemblyBlock {
    fn evaluate(&self, _: &mut EvaluatorScope) -> Result<Vec<u8>, EvaluationError> {
        return Ok(self.compiled.clone());
    }
}

impl RawMacroBlock for AssemblyBlock {
    fn allocate(
        line_number: usize,
        _args: Vec<String>,
        lines: Vec<String>,
    ) -> Result<Rc<Self>, EvaluationError> {
        let contents: String = lines.iter().fold(String::new(), |a, v| a + v + "\n");
        let compiled: Vec<u8> = compile_assembly(&contents).map_err(|e| e.at(line_number))?;
        Ok(Rc::new(Self {
            line_number,
            compiled,
        }))
    }
}
