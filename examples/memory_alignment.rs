struct A (u16, u32, u16);

struct B (u16, u16, u32);

#[repr(C)]
struct A_C (u16, u32, u16);

#[repr(C)]
struct B_C (u16, u16, u32);

pub fn main() {
    // TDIL: Rust performs memory layout optimization to some extent
    println!("A - Size: {}, Aligntment: {}", std::mem::size_of::<A>(), std::mem::align_of::<A>());
    println!("B - Size: {}, Aligntment: {}", std::mem::size_of::<B>(), std::mem::align_of::<B>());

    println!("A repr(C) - Size: {}, Aligntment: {}", std::mem::size_of::<A_C>(), std::mem::align_of::<A_C>());
    println!("B repr(C) - Size: {}, Aligntment: {}", std::mem::size_of::<B_C>(), std::mem::align_of::<B_C>());
}
