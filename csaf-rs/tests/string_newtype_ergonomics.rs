//! The generated constrained-string newtypes compare against `str` and `&str` in both
//! operand orders and borrow as `&str`, without dereferencing through the inner `String`.

use csaf::schema::csaf2_0::schema::ProductIdT as ProductIdT20;
use csaf::schema::csaf2_1::schema::ProductIdT as ProductIdT21;

#[test]
fn product_ids_compare_against_str() {
    let id: ProductIdT20 = "CSAFPID-0001".parse().expect("valid product id");
    assert_eq!(id, "CSAFPID-0001");
    assert_eq!("CSAFPID-0001", id);
    assert!(id != "CSAFPID-0002");
    assert!("CSAFPID-0002" != id);
    assert_eq!(id, *"CSAFPID-0001");
    assert_eq!(*"CSAFPID-0001", id);
    assert_eq!(id.as_ref(), "CSAFPID-0001");

    let id: ProductIdT21 = "CSAFPID-0001".parse().expect("valid product id");
    assert_eq!(id, "CSAFPID-0001");
    assert_eq!("CSAFPID-0001", id);
}
