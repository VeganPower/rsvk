use std::string;

fn main() {
    let vk_lib = std::env::var("VULKAN_SDK").unwrap();
    let s = String::from(r"cargo:rustc-link-search=") + &vk_lib + &String::from("/lib");
    println!("{}", s);
}
