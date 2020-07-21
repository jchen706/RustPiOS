use core::panic::PanicInfo;


use crate::console::kprintln;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {

    if let Some(location) = _info.location() {
        kprintln!("{:?}", _info.message());
        kprintln!("Panic occurred in file {}", location.file());
        kprintln!("Panic occurred on line {}", location.line());
        kprintln!("Panic occurred on line {}", location.column());

    } else {
        kprintln!("Panic error {:?}", _info.message());
    }



    loop {        
    }
}
