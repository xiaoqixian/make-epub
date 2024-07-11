// Date:   Thu Jul 11 17:09:32 2024
// Mail:   lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

macro_rules! auto_from {
    ($base: ty, $name: ident, $typ: ty) => {
        impl From<$typ> for $base {
            fn from(item: $typ) -> Self {
                Self::$name(item)
            }
        }
    }
}

#[derive(Debug)]
#[warn(dead_code)]
pub enum Error {
    IOError(std::io::Error),
    RegexError(regex::Error)
}

auto_from!(Error, IOError, std::io::Error);
auto_from!(Error, RegexError, regex::Error);
