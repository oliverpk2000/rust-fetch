use crate::info::{print_fmt_de_info, print_fmt_mem_info, print_fmt_os_info, print_fmt_sh_info};

mod ascii;
mod info;

fn main() {
    let logo = ascii::get_logo(info::get_os_type());
    println!("{}", logo);
    print_fmt_os_info();
    print_fmt_de_info();
    print_fmt_sh_info();
    print_fmt_mem_info()
}
