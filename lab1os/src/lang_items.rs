use core::panic::PanicInfo;
use crate::{sbi::shutdown, println};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {       // 返回值为空，即不返回，发散函数
    if let Some(location) = info.location() {
        println!(
            "Panic at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("Panicked: {}", info.message().unwrap());
    }
    shutdown()
}