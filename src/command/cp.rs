//! CP Command Abstraction
use crate::command;

pub struct CP;

command!(CP);

// impl CP {
//     pub fn cp<I, S>(args: I)
//     where
//         I: IntoIterator<Item=S> + Debug + Copy,
//         S: AsRef<OsStr>,
//     {
//         let mut cp = Command::new("cp");
//         match cp.args(args).spawn() {
//             Ok(_) => {},    // 何もしません
//             Err(err) => {
//                 // TODO: 日本語でこのエラーを表現する
//                 panic!("failed to excute cp {:?}, error: {:?}", args, err);
//             }
//         }
//     }
// }