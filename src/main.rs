use crate::info::print_fmt_info;

mod info;
mod ascii;

fn main() {
    let logo = ascii::get_logo(info::get_os_type());
    println!("{}", logo);
    print_fmt_info();
}
