use crate::NumberBytes;

#[test]
fn u64_bytes_matches() {
    assert_eq!(
        <u64 as NumberBytes>::BYTES,
        u64::BITS as usize / 8,
    )
}

#[test]
fn u32_to_ne_bytes() {
    let value: u32 = 0x12345678;
    
    let expected = value.to_ne_bytes().to_vec();
    let actual = <u32 as NumberBytes>::to_ne_bytes(value);

    assert_eq!(expected, actual)
}

#[test]
fn u32_to_be_bytes() {
    let value: u32 = 0x012345678;
    
    let expected = value.to_be_bytes().to_vec();
    let actual = <u32 as NumberBytes>::to_be_bytes(value);

    assert_eq!(expected, actual)
}

#[test]
fn u32_to_le_bytes() {
    let value: u32 = 0x012345678;
    
    let expected = value.to_le_bytes().to_vec();
    let actual = <u32 as NumberBytes>::to_le_bytes(value);

    assert_eq!(expected, actual)
}

#[test]
fn f64_to_ne_bytes() {
    let value: f64 = 0.123456789;
    
    let expected = value.to_ne_bytes().to_vec();
    let actual = <f64 as NumberBytes>::to_ne_bytes(value);

    assert_eq!(expected, actual)
}
