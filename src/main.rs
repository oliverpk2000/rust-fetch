mod info;
mod ascii;

fn main() {
    let logo = ascii::get_logo(info::get_os_type());
    println!("{}", logo);
    println!("{}", info::fmt_info_header());
    println!("{}", info::fmt_info());
}
