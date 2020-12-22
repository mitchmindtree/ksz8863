use ksz8863::miim::{self, Miim};

#[test]
fn test_bcr_default() {
    let actual: u16 = miim::Bcr::default().into();
    let expected = 0b0000_0100_0000_1000;
    assert_eq!(actual, expected);
}

#[test]
fn test_bcr_reset() {
    // All fields are RW, so resetting should reset to default state.
    let mut actual = miim::Bcr::from(0);
    assert!(actual != miim::Bcr::default());
    actual.write().reset();
    assert_eq!(actual, miim::Bcr::default());
}

#[test]
fn test_bsr_default() {
    let actual: u16 = miim::Bsr::default().into();
    let expected = 0b0001_0000_0001_1110;
    assert_eq!(actual, expected);
}

// Checks all the register default constructors.
// Run with `cargo test -- --nocapture` to see all default register state.
#[test]
fn miim_map_default() {
    let map = miim::Map::default();
    for &addr in miim::Address::ALL {
        println!("{:#?}", map[addr]);
    }
}

#[test]
fn miim_api() {
    // Rather than a `Map`, we would normally use a real MIIM interface, however for testing the
    // API its easier to read and write to a map.
    let mut miim = Miim(miim::Map::default());

    // Access the phy at addr `0`.
    let mut phy = miim.phy(0);

    // Access the Bcr register.
    let mut bcr = phy.bcr();

    // Read the value. Should be default.
    let a = bcr.read().unwrap();
    assert_eq!(a, miim::Bcr::default());

    // Overwrite the Bcr register.
    bcr.write(|w| w.enable_autoneg().clear_bit()).unwrap();
    let b = bcr.read().unwrap();
    assert!(a != b);

    // Modify the Bcr register.
    bcr.modify(|w| w.force_100().set_bit()).unwrap();
    let c = bcr.read().unwrap();
    assert!(c.read().enable_autoneg().bit_is_clear());
    assert!(c.read().force_100().bit_is_set());

    // Reset the Bcr register.
    bcr.write(|w| w.reset()).unwrap();
    let d = bcr.read().unwrap();
    assert_eq!(a, d);

    // Check non-lexical borrows are working nicely.
    assert_eq!(a, miim.phy(0).bcr().read().unwrap());
}
