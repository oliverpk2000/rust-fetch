use colored::Colorize;
use detect_desktop_environment::DesktopEnvironment;
use os_info::Type;

pub fn print_fmt_os_info() {
    let info = os_info::get();

    let os_type = info.os_type();
    let version = info.version();
    let edition = info.edition().unwrap_or("none found");
    let codename = info.codename().unwrap_or("none found");
    let bitness = info.bitness();
    let architecture = info.architecture().unwrap_or("none found");

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
    os_type
}

pub fn print_fmt_de_info() {
    match DesktopEnvironment::detect() {
        Some(de) => println!("{}: {:?}", "desktop environment".yellow(), de),
        None => println!("desktop environment: none found"),
    }
}