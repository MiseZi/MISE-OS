use core::panic::PanicInfo;
use crate::println;

#[panic_handler]
fn panic_handler(panic_info: &PanicInfo) -> ! {
    let _err = panic_info.message().unwrap();
    if let Some(location) = panic_info.location() {
        println!(
            "Paniced at {}:{} {}",
            location.file(),
            location.line(),
            _err
        );
    } else {
        println!("Panicked: {}", panic_info.message().unwrap());
    }
    loop {}
}