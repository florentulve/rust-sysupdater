use std::process::{Command};
use std::string::String;
use std::process::ExitStatus;
use std::io;


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
    
    SysUpdater{}.update().expect("Error while updating rpm");
    
    FlatpakUpdater{}.update().expect("Error while update flatpak");
    
    let rpm_pathname = "/tmp/code-insiders.rpm";
    let code_updater = CodeUpdater::with_pathname(rpm_pathname);
    code_updater.download().expect("Error while downloading");
    code_updater.update().expect("Error while installing");

    cmd!("rustup", ["update"]).status().expect("Rustup failed");

    

}
