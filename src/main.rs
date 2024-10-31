extern crate my_proc_macro;

fn main() {
    proc_macro_demo();
    derive_macro_demo();
    attribute_macro_demo();
}

// function-like macros
fn proc_macro_demo() {
    use my_proc_macro::say_hello;
    say_hello!(1, 2);
    println!("{}", hello());
}

// derive macros
use my_proc_macro::{IntoURLQueryString, URLQueryGetter};
#[derive(IntoURLQueryString, URLQueryGetter, Clone)]
pub struct MyQueryParameters {
    pub offset: u32,
    pub limit: u32,
    pub condition: String,
}

fn derive_macro_demo() {
    let params = MyQueryParameters {
        offset: 3,
        limit: 5,
        condition: "▣▣▣▣▣▣".to_owned(),
    };
    println!("{}", String::from(params.clone()));
    println!("{}", params.get_limit_query());
    println!("{}", params.get_offset_query());
    println!("{}", params.get_condition_query());
}

// attribute macros
use my_proc_macro::log_info;
#[log_info(bar)]
struct InfoStruct {}

#[log_info]
fn info_function() {}

#[log_info {3}]
enum InfoEnum {
    Warning,
}

fn attribute_macro_demo() {
    InfoStruct {};
    info_function();
    let _warning = InfoEnum::Warning;
}
