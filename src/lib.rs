use hs_bindgen::*;

/// Haskell type signatures are auto-magically inferred from Rust function
/// types! This feature could slow down compilation, and be enabled with:
/// `hs-bindgen = { ..., features = [ "full" ] }`
#[hs_bindgen]
fn greetings(name: &str) {
    println!("Hello, {name}!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
