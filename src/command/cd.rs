//! CD Command Abstraction
use std::process::Command;
use std::ffi::OsStr;
use std::fmt::Debug;

#[allow(dead_code)]
pub struct CD;

impl CD {
    // This func don't work
    #[allow(dead_code)]
    pub fn cd<S: AsRef<OsStr> + Debug + Copy>(arg: S) {
        let mut cd = Command::new("sh");
        match cd.arg("-c").arg("cd").arg(arg).spawn() {
            Ok(_) => {},    //do nothing
            Err(err) => {
                panic!("failed to excute cd {:?}, error: {:?}", arg, err);
            }
        }
    }
}