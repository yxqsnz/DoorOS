use x86_64::structures::idt::InterruptStackFrame;
use crate::{fmt::Write, sys::vga::{color::{ColorCode, ColorType}, WRITER}};

pub extern "x86-interrupt" fn handle(frame: InterruptStackFrame) {
    let mut console = WRITER.lock();
    console.color = ColorType::Custom(crate::color!(ColorCode::Blue, ColorCode::Red));
    writeln!(console, "EXCEPTION: OVERFLOW").unwrap();
    writeln!(console, "Accessed Address: {:?}", frame.instruction_pointer).unwrap();
    
    writeln!(console, "Stack: {:#?}", frame).unwrap();

    loop {}
}