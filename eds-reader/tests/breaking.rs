use eds_reader::Reader;

#[test]
pub fn breaking() {
    let data1 = [0x55, 0x55, 0x55, 0x54, 0x00];
    let data2 = [0x02, 0x01, 0x02, 0xE1, 0x81, 0x0A];
    let mut reader = Reader::<2>::new(3);
    let used = reader.recv(&data1);
    assert_eq!(used, 5);
    assert!(!reader.is_ready());
    assert_eq!(reader.get_load(), None);

    let used = reader.recv(&data2);
    assert_eq!(used, 6);
    assert!(reader.is_ready());
    assert_eq!(reader.get_load().unwrap(), &[1, 2]);
}
