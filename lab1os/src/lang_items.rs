use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {       // 返回值为空，即不返回，发散函数
    loop{}  //暂时无限循环
}