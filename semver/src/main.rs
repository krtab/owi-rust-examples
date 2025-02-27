use owi_sym::Symbolic;
use semver::Version;

fn test<const N: usize>() {
    let bytes : [u8;N] = Symbolic::symbol();
    if let Ok(string) = std::str::from_utf8(&bytes) {
        if let Ok(v1) = Version::parse(string) {
            let v2 = Version::parse(&v1.to_string()).unwrap();
            assert_eq!(v1, v2);
        }
    }
}

fn main() {
    println!("Coucou");
    // test::<5>();
}
