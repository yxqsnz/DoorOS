mod page_fault;
mod general_protection_fault;
mod overflow;
mod bound_range_exceeded;
use x86_64::structures::idt::InterruptDescriptorTable;
lazy_static::lazy_static!(
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.page_fault.set_handler_fn(page_fault::handle); 
        idt.overflow.set_handler_fn(overflow::handle);
        idt.general_protection_fault.set_handler_fn(general_protection_fault::handle);
        idt.bound_range_exceeded.set_handler_fn(bound_range_exceeded::handle);
        idt
    };
);
pub fn init() {
    IDT.load();
}
