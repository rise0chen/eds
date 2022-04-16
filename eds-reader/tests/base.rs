use eds_reader::Reader;

#[test]
pub fn base() {
    let data = [
        0x55, 0x55, 0x55, 0x54, 0x00, 0x02, 0x01, 0x02, 0xE1, 0x81, 0x0A,
    ];
    let mut reader = Reader::new(3);
    let used = reader.recv(&data);
    assert_eq!(used, 11);
    assert!(reader.is_ready());
    assert_eq!(reader.get_load().unwrap().as_ref(), &[1, 2]);
}
