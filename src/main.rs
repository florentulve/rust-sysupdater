use std::process::{Command,Stdio};
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

trait Updater{
    fn download(&self) -> io::Result<ExitStatus>;

    fn install(&self) -> io::Result<ExitStatus>;
}

struct CodeUpdater{
    pathname: String,
}

impl CodeUpdater {
    pub fn with_pathname(s: &str) -> CodeUpdater{
        CodeUpdater{pathname: String::from(s)}
    }
}

impl Updater for CodeUpdater{

    fn download(&self) -> io::Result<ExitStatus>{
        cmd!("wget", ["https://go.microsoft.com/fwlink/?LinkID=760866", "-O", &self.pathname])
            .status()
    }

    fn install(&self) -> io::Result<ExitStatus>{ 
        cmd!("sudo", ["dnf", "install", "-y", &self.pathname])
            .status()
    }
}

fn main(){
    
    let rpm_pathname = "/tmp/code-insiders.rpm";
    let code_updater = CodeUpdater::with_pathname(rpm_pathname);
    code_updater.download().expect("Error while downloading");
    code_updater.install().expect("Error while installing");

}
