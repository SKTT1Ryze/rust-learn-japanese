//! LS Command Abstraction
use crate::command;

pub struct LS;

command!(LS);


// impl LS {
//     pub fn ls<S: AsRef<OsStr> + Debug + Copy>(arg: S) {
//         let mut ls = Command::new("ls");
//         match ls.arg(arg).spawn() {
//             Ok(_) => {},    // 何もしません
//             Err(err) => {
//                 // TODO: 日本語でこのエラーを表現する
//                 panic!("failed to excute ls {:?}, error: {:?}", arg, err);
//             }
//         }
//     }
// }