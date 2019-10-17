use std::process::{Command,Stdio};
use std::path::Path;
use std::string::String;
use std::process::ExitStatus;
use std::io;

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
        Command::new("wget")
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .arg("https://go.microsoft.com/fwlink/?LinkID=760866")
            .arg("-O")
            //.arg("/dev/null")
            .arg(&self.pathname)
            .status()
    }

    fn install(&self) -> io::Result<ExitStatus>{ 
        Command::new("sudo")
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .arg("--")
            .arg("dnf")
            .arg("install")
            .arg("-y")
            .arg(&self.pathname)
            .status()
    }
}

fn main(){
    
    let rpm_pathname = "/tmp/code-insiders.rpm";
    let code_updater = CodeUpdater::with_pathname(rpm_pathname);
    code_updater.download().expect("Error while downloading");
    code_updater.install().expect("Error while installing");

}
