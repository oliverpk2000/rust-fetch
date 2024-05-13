use colored::Colorize;
use os_info::Type;

pub fn print_fmt_info() {
    let info = os_info::get();

    let os_type = info.os_type();
    let version = info.version();
    let edition = info.edition().unwrap_or("unknown/error");
    let codename = info.codename().unwrap_or("unknown/error");
    let bitness = info.bitness();
    let architecture = info.architecture().unwrap_or("unknown/error");

    println!("{}: {}", "OS".yellow(), os_type);
    println!("{}: {}", "version".yellow(), version);
    println!("{}: {}", "edition".yellow(), edition);
    println!("{}: {}", "codename".yellow(), codename);
    println!("{}: {}", "bitness".yellow(), bitness);
    println!("{}: {}", "architecture".yellow(), architecture);
}

pub fn get_os_type() -> Type {
    let info = os_info::get();
    let os_type = info.os_type();
    return os_type;
}