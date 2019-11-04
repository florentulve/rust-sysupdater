use std::process::{Command};
use std::string::String;
use std::process::ExitStatus;
use std::io;

#[macro_export]
macro_rules! cmd {
    ( $x:expr, $y:tt) => {
        {   
            Command::new($x)
                .args(&$y)
        }
    };
}

type UpdateResult = io::Result<ExitStatus>;

pub trait Update{
    fn update(&self) -> UpdateResult;
}

pub trait Download{
    fn download(&self) -> UpdateResult;}

pub struct CodeUpdate{
    pathname: String,
}

pub struct SysUpdate{
}

pub struct FlatpakUpdate{
}

pub struct RustupUpdate{
}

impl CodeUpdate {
    pub fn with_pathname(s: &str) -> CodeUpdate{
        CodeUpdate{pathname: String::from(s)}
    }
}

impl Update for CodeUpdate{

    fn update(&self) -> UpdateResult{ 
        cmd!("sudo", ["dnf", "install", "-y", &self.pathname])
            .status()
    }
}

impl Download  for CodeUpdate{
    fn download(&self) -> UpdateResult{
        cmd!("wget", ["https://go.microsoft.com/fwlink/?LinkID=760866", "-O", &self.pathname])
            .status()
    }
}

impl Update for SysUpdate{

    fn update(&self) -> UpdateResult{ 
        cmd!("sudo", ["dnf", "update", "--refresh"])
            .status()
    }
}

impl Update for FlatpakUpdate{

    fn update(&self) -> UpdateResult{ 
        cmd!("flatpak", ["update"])
            .status()
    }
}

impl Update for RustupUpdate{

    fn update(&self) -> UpdateResult{ 
        cmd!("rustup", ["update"])
            .status()
    }
}


pub fn update(u: impl Update) -> UpdateResult{
    u.update()
}

pub fn download_and_update(u: impl Update+Download) -> UpdateResult{
    u.download()
        .and(u.update())
}