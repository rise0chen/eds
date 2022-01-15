use eds::{Decoder, Encoder};

#[test]
pub fn empty() {
    let eds_encoder = Encoder::new(3, &[]);
    let mut data = [0; 100];
    let data_len = eds_encoder.encode(&mut data).unwrap();
    let except = [0x55, 0x55, 0x55, 0x54, 0x00, 0x00, 0xFF, 0xFF, 0x0A];
    assert_eq!(&data[..data_len], &except);

    let eds_decoder = Decoder::new(3);
    let load = eds_decoder.get_load(&data[..data_len]).unwrap();
    assert_eq!(&load, &[]);
}
