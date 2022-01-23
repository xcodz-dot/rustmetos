use std::env::consts::EXE_SUFFIX;
use std::error::Error;
use std::fs;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct ExecError {
    pub err: String,
}

impl From<&str> for ExecError {
    fn from(s: &str) -> ExecError {
        ExecError {
            err: s.to_string().to_owned(),
        }
    }
}

impl std::fmt::Display for ExecError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Execution Error: {}", self.err)
    }
}

impl Error for ExecError {}

pub fn input(prompt: &str) -> String {
    rustmetos_core::input(prompt)
}

pub fn home() -> String {
    rustmetos_core::init();
    rustmetos_core::home().to_string().to_owned()
}

pub fn path(path: &str) -> String {
    home() + path
}

pub fn exec(name: &str, args: Vec<&str>) -> Result<(), Box<dyn Error>> {
    let name: &str = &(String::from(name) + EXE_SUFFIX);
    let path = fs::read_to_string(path("/conf/path"))?;
    let path = path.split(";");
    let mut binary_name: Option<String> = None;

    for x in path {
        let files: Vec<String> = fs::read_dir(crate::api::path(x))
            .unwrap()
            .map(|file| file.unwrap().path())
            .filter_map(|file| {
                if file.is_file() {
                    Some(String::from(
                        file.file_name()
                            .unwrap_or(std::ffi::OsStr::new(""))
                            .to_str()
                            .unwrap_or(""),
                    ))
                } else {
                    None
                }
            })
            .collect();
        if files.contains(&String::from(name)) {
            binary_name = Some(crate::api::path(&(String::from(x) + "/" + name)));
        }
    }

    match binary_name {
        Some(name) => {
            #[cfg(unix)]
            {
                Command::new("chmod").arg("+x").arg(&name).spawn()?.wait()?;
            }

            Command::new(name).args(args).spawn()?.wait()?;
        }
        None => {
            return Err(ExecError::from("Executable not found in path").into());
        }
    }

    Ok(())
}
