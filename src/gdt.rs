use x86_64::{
    VirtAddr,
    structures::tss::TaskStateSegment,
    structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector},
};
use lazy_static::lazy_static;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector =gdt.add_entry(Descriptor::kernel_code_segment());
        let tts_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors { code_selector, tts_selector })
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tts_selector: SegmentSelector,
}

pub fn init() {
    use x86_64::instructions::{
        tables::load_tss,
        segmentation::{CS, Segment}
    };

    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tts_selector);
    }
}
