use macros::my_attr_macro;

#[my_attr_macro]
fn main() {
    let s = AutoStruct { auto_field: 32 };
    println!("{}", s.auto_field);
}
