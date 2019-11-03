use std::process::{Command};
use std::string::String;
use std::process::ExitStatus;
use std::{io};

#[macro_export]
macro_rules! cmd {
    ( $x:expr, $y:tt) => {
        {   
            Command::new($x)
                .args(&$y)
        }
    };
}

pub trait SimpleUpdater{
    fn update(&self) -> io::Result<ExitStatus>;
}

pub trait ExternalUpdater{
    fn download(&self) -> io::Result<ExitStatus>;
}

pub struct CodeUpdater{
    pathname: String,
}

pub struct SysUpdater{
}

pub struct FlatpakUpdater{
}

impl CodeUpdater {
    pub fn with_pathname(s: &str) -> CodeUpdater{
        CodeUpdater{pathname: String::from(s)}
    }
}

impl ExternalUpdater for CodeUpdater{

    fn download(&self) -> io::Result<ExitStatus>{
        cmd!("wget", ["https://go.microsoft.com/fwlink/?LinkID=760866", "-O", &self.pathname])
            .status()
    }
}

impl SimpleUpdater for CodeUpdater{

    fn update(&self) -> io::Result<ExitStatus>{ 
        cmd!("sudo", ["dnf", "install", "-y", &self.pathname])
            .status()
    }
}

impl SimpleUpdater for SysUpdater{

    fn update(&self) -> io::Result<ExitStatus>{ 
        cmd!("sudo", ["dnf", "update", "--refresh"])
            .status()
    }
}

impl SimpleUpdater for FlatpakUpdater{

    fn update(&self) -> io::Result<ExitStatus>{ 
        cmd!("flatpak", ["update"])
            .status()
    }
}