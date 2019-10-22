use std::process::{Command};
use std::string::String;
use std::process::ExitStatus;
use std::{io};
use clap::{App, Arg}; 
use env_logger::Env;

#[macro_use] 
extern crate log;

macro_rules! cmd {
    ( $x:expr, $y:tt) => {
        {   
            Command::new($x)
                .args(&$y)
        }
    };
}

trait SimpleUpdater{
    fn update(&self) -> io::Result<ExitStatus>;
}

trait ExternalUpdater{
    fn download(&self) -> io::Result<ExitStatus>;
}

struct CodeUpdater{
    pathname: String,
}

struct SysUpdater{
}

struct FlatpakUpdater{
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

fn main(){
    
    let env = Env::default()
        .filter_or("SYSUPDATER_LOG_LEVEL", "info")
        .write_style_or("SYSUPDATER_LOG_STYLE", "always");

    env_logger::init_from_env(env);
    
    let matches = App::new("My Super Program")
        .version("1.0")
        .author("Florent U. <florentulve@gmail.com>")
        .about("SysUpdater")
        .arg(Arg::with_name("flatpak")
            .long("flatpak")
            .help("Update Flatpak")
            .takes_value(false))
        .arg(Arg::with_name("rpm")
            .long("rpm")
            .help("Update Rpm")
            .takes_value(false))
        .arg(Arg::with_name("vscode")
            .long("vscode")
            .help("Update Vscode")
            .takes_value(false))
        .arg(Arg::with_name("rustup")
            .long("rustup")
            .help("Update Rustup")
            .takes_value(false))
        .get_matches();

    if matches.is_present("rpm"){
        info!("Updating rpm...");
        match (SysUpdater{}.update()) {
            Ok(status) => info!("rpm updated with status {}", status),
            Err(err) => error!("rpm update failed with {}", err)
        }
    }
    if matches.is_present("flatpak"){
        info!("Updating flatpak...");
        match (FlatpakUpdater{}.update()) {
            Ok(status) => info!("flatpak updated with {}", status),
            Err(err) => error!("flatpak update failed with {}", err)
        }
    }
    
    if matches.is_present("vscode"){
        info!("Updating vscode...");
        let rpm_pathname = "/tmp/code-insiders.rpm";
        let code_updater = CodeUpdater::with_pathname(rpm_pathname);
        match code_updater.download()
                .and(code_updater.update()){
            Ok(status) => info!("vscode updated with {}", status),
            Err(err) => error!("vscode update failed with {}", err)
        }
    }

    if matches.is_present("rustup"){
        info!("Updating rustup...");
        match cmd!("rustup", ["update"]).status(){
            Ok(status) => info!("Rustup updated with {}", status),
            Err(err) => error!("Rustup update failed with {}", err)
        }
    }

}
