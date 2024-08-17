use rustrover_proc_macro_bug::MyComponent;

trait MyComponentTrait {
    fn foo();
}

#[derive(MyComponent)]
#[require(AnotherStruct(create_another))]
struct MyStruct {
    _value: String,
}

struct AnotherStruct(u32);

fn create_another() -> AnotherStruct {
    AnotherStruct(42)
}

fn main() {
    MyStruct::foo();
}



