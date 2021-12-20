use x86_64::structures::idt::InterruptStackFrame;

pub extern "x86-interrupt" fn handle(frame: InterruptStackFrame) {
    use crate::{
        fmt::Write,
        sys::vga::{
            color::{ColorCode, ColorType},
            WRITER,
        },
    };

    let mut console = WRITER.lock();
    console.color = ColorType::Custom(crate::color!(ColorCode::Blue, ColorCode::Red));
    writeln!(console, "EXCEPTION: BOUND RANGE EXCEEDED").unwrap();
    writeln!(
        console,
        "Accessed Address: {:?}",
        x86_64::registers::control::Cr2::read()
    )
    .unwrap();

    writeln!(console, "Stack: {:#?}", frame).unwrap();

    loop {}
}
