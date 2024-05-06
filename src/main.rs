mod info;
mod ascii;

fn main() {
    let logo = ascii::get_logo(info::get_os_type());
    println!("{}", info::fmt_info());
    println!("{}", logo);
}
