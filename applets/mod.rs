mod cal;
mod cat;
mod clear;
mod dirname;
mod echo;
mod _false;
mod head;
mod mkdir;
mod pwd;
mod rmdir;
mod seq;
mod sleep;
mod tee;
mod _true;
mod wc;
mod which;
mod yes;

pub fn find_applet(name: &str) -> Option<fn(&[~str])> {
    match name {
        "cal" => Some(cal::main),
        "cat" => Some(cat::main),
        "clear" => Some(clear::main),
        "dirname" => Some(dirname::main),
        "echo" => Some(echo::main),
        "false" => Some(_false::main),
        "head" => Some(head::main),
        "mkdir" => Some(mkdir::main),
        "pwd" => Some(pwd::main),
        "rmdir" => Some(rmdir::main),
        "seq" => Some(seq::main),
        "sleep" => Some(sleep::main),
        "tee" => Some(tee::main),
        "true" => Some(_true::main),
        "wc" => Some(wc::main),
        "which" => Some(which::main),
        "yes" => Some(yes::main),
        _ => None
    }
}
