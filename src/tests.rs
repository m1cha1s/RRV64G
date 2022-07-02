extern crate std;

use std::println;

use crate::cpu;

#[test]
fn it_works() {
    let result = 2 + 2;
    println!("Hello world!");
    assert_eq!(result, 4);
}

#[test]
fn test_decoding() {
    let inst = cpu::Cpu::decode(0x0105053b);

    if let Ok(inst) = inst {
        println!("{:?}", inst);
    } else {
        println!("Error");
    }

    assert_eq!(1, 0);
}
