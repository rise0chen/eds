use eds_writer::Writer;

#[test]
pub fn base() {
    let data = [
        0x55, 0x55, 0x55, 0x54, 0x00, 0x02, 0x01, 0x02, 0xE1, 0x81, 0x0A,
    ];
    let mut writer = Writer::new(3);
    let mut load = writer.get_load().unwrap();
    assert!(writer.get_load().is_none());
    load.extend_from_slice(&[1, 2]);
    assert_eq!(writer.get_data(load), data);
}
