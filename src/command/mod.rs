//! Command Implementation
//! すべてのコマンドはここに置かれること

pub mod cd;
pub mod ls;
pub mod cp;

#[macro_export]
macro_rules! command {
    ($t:ty) => {
        use std::process::Command;
        use std::ffi::OsStr;
        use std::fmt::Debug;
        impl $t {
            pub fn excute<I, S>(args: I)
            where
                I: IntoIterator<Item=S> + Debug + Copy,
                S: AsRef<OsStr>,
            {
                let type_name = std::any::type_name::<$t>();
                let typename: Vec<&str> = type_name.split("::").collect();
                let command_name = typename[typename.len() - 1].to_lowercase();
                let mut command = Command::new(&command_name);
                match command.args(args).spawn() {
                    Ok(_) => {},// 何もしません
                    Err(err) => {
                        // TODO: 日本語でこのエラーを表現する
                        panic!("failed to excute command {} {:?}, error: {:?}", &command_name, args, err);
                    }
                }

            }
        }
    };
}