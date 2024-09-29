// examples/id_pin.rs

// #!/usr/bin/env rust-script
// //! ```cargo
// //! [dependencies]
// //! devela = { path = "..", features = ["alloc"] }
// //! ```

use devela::_all::*;

fn main() {
    let mut data1 = 0u8;
    let id1 = IdPin::new(&mut data1);
    // let id1a = IdPin::new(&mut data1);// can't double borrow :)
    let mut data2 = 0u8;
    let id2 = IdPin::new(&mut data2);

    println!("IdPin (stack)");
    println!("id1: {id1:?}");
    println!("id2: {id2:?}");

    #[cfg(feature = "alloc")]
    {
        println!("IdPinBox (heap)");
        let id3 = IdPinBox::new();
        let id4 = id3.clone();
        println!("id3: {id3:?}");
        println!("id4: {id4:?}");
    }
}
