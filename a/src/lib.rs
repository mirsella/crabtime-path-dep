#[crabtime::expression]
fn test() {
    let v = b::B("Hello".to_string());
    crabtime::output! { v }
}

const A: b::B = test!();
