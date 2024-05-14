use colored::Colorize;
use detect_desktop_environment::DesktopEnvironment;
use os_info::Type;
use sysinfo::System;

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
    info.os_type()
}

pub fn print_fmt_de_info() {
    match DesktopEnvironment::detect() {
        Some(de) => println!("{}: {:?}", "desktop environment".yellow(), de),
        None => println!("desktop environment: none found"),
    }
}

pub fn print_fmt_sh_info() {
    let system = sysinfo::System::new_with_specifics(
        sysinfo::RefreshKind::new().with_processes(sysinfo::ProcessRefreshKind::new()),
    );
    let my_pid = sysinfo::get_current_pid().expect("unable to get PID of the current process");
    let parent_pid = system
        .process(my_pid)
        .expect("no self process?")
        .parent()
        .expect("unable to get parent process");
    let parent_process = system
        .process(parent_pid)
        .expect("unable to get parent process");
    let parent_name = parent_process.name();

    println!("{}: {}", "parent process".yellow(), parent_name);
}

pub fn print_fmt_mem_info() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let total_mem_gb = sys.total_memory() as f64 / 1000000000.0;
    let used_mem_gb = sys.used_memory() as f64 / 1000000000.0;

    let total_swap_gb = sys.total_swap() as f64 / 1000000000.0;
    let used_swap_gb = sys.used_swap() as f64 / 1000000000.0;


    println!("{}: {:.2} GB / {:.2} GB", "memory".yellow(), used_mem_gb, total_mem_gb);
    println!("{}: {:.2} GB / {:.2} GB", "swap".yellow(), used_swap_gb, total_swap_gb);
}