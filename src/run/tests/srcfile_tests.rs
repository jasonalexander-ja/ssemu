use super::srcfile::*;


#[test]
fn test_decode_word() {
    let source = vec![0x0F, 0x0F, 0xF0, 0xF0,   0x0F, 0x0F, 0xFF, 0xF0];
    let func = read_word(&source);
    let result = (func)(0);
    // assert_eq!((func)(0), 
    //     0x0F0F_F0F0
    // )
}

