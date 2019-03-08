mod foo;
mod bar;

fn main() {
    println!("{}", foo::make_string());
    println!("{}", bar::make_string_backwards());
}
