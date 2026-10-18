use std::mem::{align_of, offset_of, size_of};

#[repr(C)]
#[derive(Debug)]
struct Example {
    a: u8,
    b: u32,
    c: u16,
}

#[repr(C)]
#[derive(Debug)]
struct ExampleC {
    a: u8,
    b: u32,
}

#[allow(dead_code)]
struct DefaultRepr {
    a: u8,
    b: u32,
}

fn print_primitive_types() {
    println!("memcop | Type Layout\n");
    println!("Primitive Types");
    println!("---------------");
    println!(
        "u8       size: 1 byte    alignment: {} byte",
        align_of::<u8>()
    );
    println!(
        "u16      size: {} bytes   alignment: {} byte",
        size_of::<u16>(),
        align_of::<u16>()
    );
    println!(
        "u32      size: {} bytes   alignment: {} byte",
        size_of::<u32>(),
        align_of::<u32>()
    );
    println!(
        "u64      size: {} bytes   alignment: {} byte",
        size_of::<u64>(),
        align_of::<u64>()
    );
    println!(
        "u128     size: {} bytes   alignment: {} byte",
        size_of::<u128>(),
        align_of::<u128>()
    );
    println!(
        "i32      size: {} bytes   alignment: {} byte",
        size_of::<i32>(),
        align_of::<i32>()
    );
    println!(
        "f32      size: {} bytes   alignment: {} byte",
        size_of::<f32>(),
        align_of::<f32>()
    );
    println!(
        "f64      size: {} bytes   alignment: {} byte",
        size_of::<f64>(),
        align_of::<f64>()
    );
    println!(
        "bool     size: {} byte     alignment: {} byte",
        size_of::<bool>(),
        align_of::<bool>()
    );
    println!(
        "char     size: {} bytes   alignment: {} byte",
        size_of::<char>(),
        align_of::<char>()
    );
    println!(
        "usize    size: {} bytes   alignment: {} byte (target-dependent)",
        size_of::<usize>(),
        align_of::<usize>()
    );
    println!(
        "isize    size: {} bytes   alignment: {} byte (target-dependent)",
        size_of::<isize>(),
        align_of::<isize>()
    );
    println!();
    println!(
        "&u8      size: {} bytes   alignment: {} byte (target-dependent)",
        size_of::<&u8>(),
        align_of::<&u8>()
    );
    println!(
        "&u64     size: {} bytes   alignment: {} byte (target-dependent)",
        size_of::<&u64>(),
        align_of::<&u64>()
    );
    println!(
        "*const u64 size: {} bytes   alignment: {} byte (target-dependent)",
        size_of::<*const u64>(),
        align_of::<*const u64>()
    );
    println!(
        "&[u8]    size: {} bytes   alignment: {} byte (target-dependent)",
        size_of::<&[u8]>(),
        align_of::<&[u8]>()
    );
    println!(
        "&str     size: {} bytes   alignment: {} byte (target-dependent)",
        size_of::<&str>(),
        align_of::<&str>()
    );
    println!();
}

fn print_structs() {
    println!("Structs");
    println!("------");
    println!("ExampleC (#[repr(C)])");
    println!("  size:      {} bytes", size_of::<ExampleC>());
    println!("  alignment: {} bytes", align_of::<ExampleC>());
    println!("  Field       Offset");
    println!("  -----       ------");
    println!("  a           {} bytes", offset_of!(ExampleC, a));
    println!("  b           {} bytes", offset_of!(ExampleC, b));
    println!();
    println!("DataC (#[repr(C)]) / Example-like as in README");
    println!("Example");
    println!("  size:      {} bytes", size_of::<Example>());
    println!("  alignment: {} bytes", align_of::<Example>());
    println!("  Field       Offset");
    println!("  -----       ------");
    println!("  a           {} bytes", offset_of!(Example, a));
    println!("  b           {} bytes", offset_of!(Example, b));
    println!("  c           {} bytes", offset_of!(Example, c));
    println!();
    println!("DefaultRepr (default Rust repr)");
    println!("  size:      {} bytes", size_of::<DefaultRepr>());
    println!("  alignment: {} bytes", align_of::<DefaultRepr>());
    println!("  (Field order not guaranteed under default repr)");
    println!();
}

pub fn run(r#type: Option<&str>) {
    match r#type {
        Some(t) => match t {
            "u8" | "U8" => println!(
                "u8       size: 1 byte    alignment: {} byte",
                align_of::<u8>()
            ),
            "u16" | "U16" => println!(
                "u16      size: {} bytes   alignment: {} byte",
                size_of::<u16>(),
                align_of::<u16>()
            ),
            "u32" | "U32" => println!(
                "u32      size: {} bytes   alignment: {} byte",
                size_of::<u32>(),
                align_of::<u32>()
            ),
            "u64" | "U64" => println!(
                "u64      size: {} bytes   alignment: {} byte",
                size_of::<u64>(),
                align_of::<u64>()
            ),
            "u128" | "U128" => println!(
                "u128     size: {} bytes   alignment: {} byte",
                size_of::<u128>(),
                align_of::<u128>()
            ),
            "i32" | "I32" => println!(
                "i32      size: {} bytes   alignment: {} byte",
                size_of::<i32>(),
                align_of::<i32>()
            ),
            "f32" | "F32" => println!(
                "f32      size: {} bytes   alignment: {} byte",
                size_of::<f32>(),
                align_of::<f32>()
            ),
            "f64" | "F64" => println!(
                "f64      size: {} bytes   alignment: {} byte",
                size_of::<f64>(),
                align_of::<f64>()
            ),
            "bool" | "Bool" => println!(
                "bool     size: {} byte     alignment: {} byte",
                size_of::<bool>(),
                align_of::<bool>()
            ),
            "char" | "Char" => println!(
                "char     size: {} bytes   alignment: {} byte",
                size_of::<char>(),
                align_of::<char>()
            ),
            "usize" | "USIZE" => println!(
                "usize    size: {} bytes   alignment: {} byte (target-dependent)",
                size_of::<usize>(),
                align_of::<usize>()
            ),
            "isize" | "ISIZE" => println!(
                "isize    size: {} bytes   alignment: {} byte (target-dependent)",
                size_of::<isize>(),
                align_of::<isize>()
            ),
            "Example" | "example" => {
                println!("Example");
                println!("  size:      {} bytes", size_of::<Example>());
                println!("  alignment: {} bytes", align_of::<Example>());
                println!("  Field       Offset");
                println!("  -----       ------");
                println!("  a           {} bytes", offset_of!(Example, a));
                println!("  b           {} bytes", offset_of!(Example, b));
                println!("  c           {} bytes", offset_of!(Example, c));
            }
            _ => {
                println!("memcop | Type Layout\n");
                println!("Unsupported type requested. Showing supported types instead.\n");
                print_primitive_types();
                print_structs();
            }
        },
        None => {
            print_primitive_types();
            print_structs();
        }
    }
}
