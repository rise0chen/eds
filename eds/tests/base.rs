use eds::{Decoder, Encoder};

#[test]
pub fn base() {
    let eds_encoder = Encoder::new(3, &[1, 2]);
    let mut data = [0; 100];
    let data_len = eds_encoder.encode(&mut data).unwrap();
    let except = [
        0x55, 0x55, 0x55, 0x54, 0x00, 0x02, 0x01, 0x02, 0xE1, 0x81, 0x0A,
    ];
    assert_eq!(&data[..data_len], &except);

    let eds_decoder = Decoder::new(3);
    let load = eds_decoder.get_load(&data[..data_len]).unwrap();
    assert_eq!(&load, &[1, 2]);

    let eds_decoder = Decoder::new(2);
    let load = eds_decoder.get_load(&data[..data_len]).unwrap();
    assert_eq!(&load, &[1, 2]);

    let eds_decoder = Decoder::new(4);
    assert!(eds_decoder.get_load(&data[..data_len]).is_err());
}
