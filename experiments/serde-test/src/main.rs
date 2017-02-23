extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate serde_derive;


#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Debug, PartialEq)]
enum SomeEnum {
    Foo,
    Bar(i32),
    Baz { a: i32, b: bool },
    Boo { c: SomeStruct },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct SomeStruct {
    a: i32,
    b: bool,
}



fn main() {
    let a = SomeEnum::Foo;
    let b = SomeEnum::Bar(42);
    let c = SomeEnum::Baz { a: 42, b: true };
    let d = SomeEnum::Boo { c: SomeStruct { a: 24, b: false } };
    let aj = serde_json::to_string(&a).unwrap();
    let bj = serde_json::to_string(&b).unwrap();
    let cj = serde_json::to_string(&c).unwrap();
    let dj = serde_json::to_string(&d).unwrap();
    let ad: Result<SomeEnum, _> = serde_json::from_str(&aj);
    let bd: Result<SomeEnum, _> = serde_json::from_str(&bj);
    let cd: Result<SomeEnum, _> = serde_json::from_str(&cj);
    let dd: Result<SomeEnum, _> = serde_json::from_str(&dj);
    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
    println!("d: {:?}", d);
    println!("aj: {:?}", aj);
    println!("bj: {:?}", bj);
    println!("cj: {:?}", cj);
    println!("dj: {:?}", dj);
    println!("ad: {:?}", ad);
    println!("bd: {:?}", bd);
    println!("cd: {:?}", cd);
    println!("dd: {:?}", dd);
    assert_eq!(a, ad.unwrap());
    assert_eq!(b, bd.unwrap());
    assert_eq!(c, cd.unwrap());
    assert_eq!(d, dd.unwrap());
}
