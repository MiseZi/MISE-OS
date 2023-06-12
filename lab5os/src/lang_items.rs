use core::panic::PanicInfo;
use crate::{sbi::shutdown, error};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {       // 返回值为空，即不返回，发散函数
    if let Some(location) = info.location() {
        error!(
            "Panic at {}:{} {}",
            locatioman.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        error!("Panicked: {}", info.message().unwrap());
    }
    shutdown(true)
}