trait DoSomething {
    fn do_something(&self);
}

impl DoSomething for i32 {
    fn do_something(&self) {
        println!("Doing something with {}", self);
    }
}

impl DoSomething for f64 {
    fn do_something(&self) {
        println!("Doing something with {}", self);
    }
}

struct MyStruct<TType: DoSomething> {
    value: TType,
}

impl<TType: DoSomething> MyStruct<TType> {
    fn new(value: TType) -> Self {
        MyStruct { value }
    }

    fn do_this(&self) {
        self.value.do_something();
    }
}

fn main() {
    let num = 42;
    let struct1 = MyStruct::new(num);
    struct1.do_this();

    let num2: f64 = 3.14;
    let struct2 = MyStruct::new(num2);
    struct2.do_this();

    // this would error as i8 does not implement the DoSomething trait
    // let num3: i8 = 42;
    // let struct3 = MyStruct::new(num3);
    // struct3.do_this();
}
