use super::*;


#[test]
fn test_format_word() {
    assert_eq!(format_word(0x0F0FF0F0), vec![0x0F, 0x0F, 0xF0, 0xF0])
}

#[test]
fn test_format_data() {
    assert_eq!(
        format_data(vec![0x0F0F_F0F0, 0x0F0F_FFF0]), 
        vec![
            0x0F, 0x0F, 0xF0, 0xF0,
            0x0F, 0x0F, 0xFF, 0xF0
        ]
    )
}
