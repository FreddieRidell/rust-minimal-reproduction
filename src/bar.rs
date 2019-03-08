mod foo;

pub fn make_string_backwards() -> String {
    String.from(foo::make_string()).split("").rev().join("")
}
