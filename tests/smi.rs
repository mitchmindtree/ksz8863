use ksz8863::smi::{self, Smi};

// Run with `cargo test -- --nocapture`
#[test]
fn smi_map_default() {
    let map = smi::Map::default();
    for &addr in smi::Address::ALL {
        println!("{:#?}", map[addr]);
    }
}

#[test]
fn smi_api() {
    // Rather than a `Map`, we would normally use a real SMI interface, however for testing the API
    // its easier to read and write to a map.
    let mut smi = Smi(smi::Map::default());

    // Access the Bcr register.
    let mut gc1 = smi.gc1();

    // Read the value. Should be default.
    let a = gc1.read().unwrap();
    assert_eq!(a, smi::Gc1::default());

    // Overwrite the Gc1 register.
    gc1.write(|w| w.aging().clear_bit()).unwrap();
    let b = gc1.read().unwrap();
    assert!(a != b);

    // Modify the Gc1 register.
    gc1.modify(|w| w.aggressive_back_off().set_bit()).unwrap();
    let c = gc1.read().unwrap();
    assert!(c.read().aging().bit_is_clear());
    assert!(c.read().aggressive_back_off().bit_is_set());

    // Reset the Gc1 register.
    gc1.write(|w| w.reset()).unwrap();
    let d = gc1.read().unwrap();
    assert_eq!(a, d);

    // Check non-lexical borrows are working nicely.
    assert_eq!(a, smi.gc1().read().unwrap());
}
