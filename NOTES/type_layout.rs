// TYPE LAYOUT NOTES
// =================
//
// Study reference:
// https://doc.rust-lang.org/reference/type-layout.html
// https://doc.rust-lang.org/reference/dynamically-sized-types.html
//
// Goal: understand size, alignment, padding, pointers, arrays, DSTs,
// representations, enums, unions, packed fields, and transparent wrappers.
//
// Run:
//   rustc type_layout_notes.rs
//   ./type_layout_notes
//
// Important:
// - Exact layout details can depend on the target platform.
// - The default Rust representation does NOT promise field order.
// - Do not treat this file as a guide to transmuting arbitrary bytes.
// - ABI and FFI compatibility are separate topics to study afterwards.

use std::marker::PhantomData;
use std::mem::{align_of, size_of};

// ---------------------------------------------------------------------
// 1. BASIC TYPES: SIZE
// ---------------------------------------------------------------------
//
// Size = how many bytes a value of the type occupies.
//
// A byte is 8 bits.
//
// The sizes below are guaranteed by Rust's primitive type layout rules.
fn show_primitive_sizes() {
    println!("u8:   {} byte", size_of::<u8>());
    println!("u16:  {} bytes", size_of::<u16>());
    println!("u32:  {} bytes", size_of::<u32>());
    println!("u64:  {} bytes", size_of::<u64>());
    println!("u128: {} bytes", size_of::<u128>());
    println!("i32:  {} bytes", size_of::<i32>());
    println!("f32:  {} bytes", size_of::<f32>());
    println!("f64:  {} bytes", size_of::<f64>());
    println!("bool: {} byte", size_of::<bool>());
    println!("char: {} bytes", size_of::<char>());

    // usize/isize are pointer-sized integer types:
    // typically 8 bytes on 64-bit targets and 4 bytes on 32-bit targets.
    println!("usize: {} bytes on this target", size_of::<usize>());
    println!("isize: {} bytes on this target", size_of::<isize>());
}

// ---------------------------------------------------------------------
// 2. ALIGNMENT
// ---------------------------------------------------------------------
//
// Alignment = which memory addresses a value is allowed to start at.
//
// For alignment 4, valid starting addresses are multiples of 4:
// 1000, 1004, 1008, 1012, ...
//
// Do not assume alignment always equals size. For example, u128 is always
// 16 bytes, but its alignment can differ between target platforms.
//
// A type's size is a multiple of its alignment. A zero-sized type's size
// is 0, which is a multiple of every valid alignment.
fn show_alignments() {
    println!("u8 alignment:   {}", align_of::<u8>());
    println!("u32 alignment:  {}", align_of::<u32>());
    println!("u64 alignment:  {}", align_of::<u64>());
    println!("u128 alignment: {}", align_of::<u128>());
}

// ---------------------------------------------------------------------
// 3. PADDING AND #[repr(C)]
// ---------------------------------------------------------------------
//
// #[repr(C)] asks Rust to use the C representation's layout rules.
//
// On a target where u8 has alignment 1 and u32 has alignment 4:
//
// ExampleC:
//   a: offset 0, size 1
//   padding: offsets 1, 2, 3
//   b: offset 4, size 4
//   total size: 8 bytes
//   alignment: 4 bytes
//
// Padding is unused space inserted so fields and the overall struct meet
// alignment requirements. Padding bytes are not extra fields.
#[repr(C)]
struct ExampleC {
    a: u8,
    b: u32,
}

// A second C-layout example. Assuming the usual alignments:
// a offset 0 (1 byte), 3 bytes padding, b offset 4 (4 bytes),
// c offset 8 (2 bytes), 2 bytes tail padding => total 12 bytes.
#[repr(C)]
struct DataC {
    a: u8,
    b: u32,
    c: u16,
}

fn show_struct_layout_sizes() {
    println!("ExampleC size: {}", size_of::<ExampleC>());
    println!("ExampleC alignment: {}", align_of::<ExampleC>());
    println!("DataC size: {}", size_of::<DataC>());
    println!("DataC alignment: {}", align_of::<DataC>());

    // For known fields, offset_of! can inspect field offsets.
    // This macro is available on modern stable Rust.
    println!("ExampleC.a offset: {}", std::mem::offset_of!(ExampleC, a));
    println!("ExampleC.b offset: {}", std::mem::offset_of!(ExampleC, b));
}

// ---------------------------------------------------------------------
// 4. DEFAULT RUST REPRESENTATION VS #[repr(C)]
// ---------------------------------------------------------------------
//
// Without a repr attribute, a struct uses the default Rust representation.
//
// Rust guarantees proper field alignment and that the struct alignment is
// at least the maximum field alignment. However, it does NOT guarantee
// declaration order for fields under the default Rust representation.
//
// Do not rely on exact field offsets or size for a default-repr struct
// unless another language guarantee specifically applies.
//
// #[repr(C)] gives the struct C-style field layout rules: fields are laid
// out in declaration order with the required padding.
//
// Note: #[repr(C)] on an outer struct does not change the representation
// of a nested inner struct. Each type has its own representation.
struct DefaultRepr {
    a: u8,
    b: u32,
}

struct InnerDefault {
    a: u8,
    b: u32,
}

#[repr(C)]
struct OuterC {
    first: u8,
    inner: InnerDefault, // InnerDefault still has default Rust repr.
}

// ---------------------------------------------------------------------
// 5. SIZED, ?SIZED, AND DYNAMICALLY SIZED TYPES (DSTs)
// ---------------------------------------------------------------------
//
// Sized means the type has a size known at compile time.
//
// Examples:
//   u32
//   [u8; 10]  // array of exactly 10 bytes
//   ExampleC
//
// DST = dynamically sized type. Its size is not fixed in the type itself.
//
// Examples:
//   [u8]      // slice type, length not included in the type
//   str       // UTF-8 string slice type
//   dyn Animal // trait object type
//
// A DST normally has to be used behind a pointer/reference, e.g. &[u8],
// &str, or &dyn Animal. The reference itself is sized.
//
// Generic T is Sized by default:
fn print_size<T: Sized>(_value: T) {
    println!("T size: {}", size_of::<T>());
}

// ?Sized relaxes the implicit Sized bound. T can be Sized or unsized.
// A reference to T is used because a DST cannot be passed by value like
// an ordinary sized local value.
fn inspect<T: ?Sized>(_value: &T) {
    println!("Inspected a value through a reference.");
}

// ---------------------------------------------------------------------
// 6. ARRAYS, SLICES, STRINGS
// ---------------------------------------------------------------------
//
// [u8; 5] is an array containing exactly five u8 values.
// [u8] is a slice type whose length can vary.
//
// Arrays store their elements contiguously. For [T; N], size is
// size_of::<T>() * N, and alignment is the element type's alignment.
fn show_arrays_and_slices() {
    let array: [u32; 3] = [10, 20, 30];
    let slice: &[u32] = &array[1..];

    println!("array size: {}", size_of_val(&array));
    println!("slice reference size: {}", size_of::<&[u32]>());
    println!("slice length: {}", slice.len());

    // str is dynamically sized UTF-8 string data.
    // &str is a sized reference that carries data pointer + length on
    // common targets. A UTF-8 byte length is not always a character count.
    let text: &str = "é";
    println!("string byte length: {}", text.len()); // 2
    println!("Unicode scalar count: {}", text.chars().count()); // 1

    let bytes: &[u8] = &[10, 20, 30];
    println!("byte slice length: {}", bytes.len());
}

use std::mem::size_of_val;

// ---------------------------------------------------------------------
// 7. ZERO-SIZED TYPES (ZSTs)
// ---------------------------------------------------------------------
//
// A ZST occupies zero bytes but can still affect the type system.
//
// Examples: (), empty structs, structs whose fields are all ZSTs,
// and PhantomData<T>.
//
// ZSTs can be useful for marker types, type-level distinctions,
// PhantomData, and some iterator/collection implementations.
//
// ZST values do not need unique addresses.
struct Empty;
struct Marker {}

struct AlsoEmpty {
    marker: (),
}

struct Tagged<T> {
    id: u64,
    marker: PhantomData<T>, // size 0; carries type-system information
}

fn show_zero_sized_types() {
    println!(
        "() size/alignment: {}/{}",
        size_of::<()>(),
        align_of::<()>()
    );
    println!("Empty size: {}", size_of::<Empty>());
    println!("Marker size: {}", size_of::<Marker>());
    println!("AlsoEmpty size: {}", size_of::<AlsoEmpty>());
    println!(
        "PhantomData<String> size: {}",
        size_of::<PhantomData<String>>()
    );
    println!("Tagged<String> size: {}", size_of::<Tagged<String>>());
}

// ---------------------------------------------------------------------
// 8. POINTERS AND REFERENCES
// ---------------------------------------------------------------------
//
// A pointer/reference is not the value it points to.
//
// For pointers to Sized types, pointer size and alignment match usize
// on the target. So on a typical 64-bit target, &u8 and &u64 are both
// 8 bytes even though u8 is 1 byte and u64 is 8 bytes.
//
// Shared/mutable references and raw pointers to the same pointee type
// have the same layout, but their safety rules are different.
//
// &T       shared reference
// &mut T   exclusive mutable reference
// *const T raw const pointer
// *mut T   raw mutable pointer
//
// A slice reference (&[T]) and string reference (&str) commonly use a
// fat pointer: data pointer + length.
// A trait-object reference (&dyn Trait) commonly uses data pointer +
// vtable pointer. These are commonly 16 bytes on 64-bit targets, but
// do not hard-code "exactly twice usize" as a universal Rust guarantee.
trait Animal {
    fn speak(&self);
}

struct Dog;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

fn show_pointer_layouts() {
    println!("&u8 size: {}", size_of::<&u8>());
    println!("&u64 size: {}", size_of::<&u64>());
    println!("*const u64 size: {}", size_of::<*const u64>());
    println!("&[u8] size: {}", size_of::<&[u8]>());
    println!("&str size: {}", size_of::<&str>());
    println!("&dyn Animal size: {}", size_of::<&dyn Animal>());
}

// ---------------------------------------------------------------------
// 9. TUPLES AND CLOSURES
// ---------------------------------------------------------------------
//
// Tuples group values of different types. Under default Rust repr, do not
// assume field order/offsets or a stable layout across compiler versions.
//
// Closure types are compiler-generated anonymous types that store any
// captured values needed by the closure. The Reference provides no
// general layout guarantee for closure types.
fn show_tuple_and_closure_examples() {
    let tuple: (u8, u32, bool) = (20, 1000, true);
    println!(
        "tuple size on this compiler/target: {}",
        size_of_val(&tuple)
    );

    let factor = 10;
    let multiply = |x: i32| x * factor;
    println!("closure result: {}", multiply(3));
    println!(
        "closure size on this compiler/target: {}",
        size_of_val(&multiply)
    );
}

// ---------------------------------------------------------------------
// 10. ENUMS AND DISCRIMINANTS
// ---------------------------------------------------------------------
//
// An enum value is one of several variants. A discriminant identifies the
// active variant. Variants with fields also carry payload data.
//
// For a default-repr enum, do not assume an exact tag/payload layout.
// Rust may optimise enum layout, including using available invalid bit
// patterns in certain types (for example, some Option types).
enum Message {
    Quit,
    Move { x: u32, y: u32 },
    Write(String),
}

// A fieldless enum with integer representation.
// repr(u8) means the enum uses u8 as its integer representation.
// Values must be valid declared discriminants when used as this Rust enum.
#[repr(u8)]
enum Direction {
    North = 0,
    South = 1,
    East = 2,
    West = 3,
}

// repr(C) enum with payload: C-style tagged-enum model (tag + payload
// storage). The exact total size still follows target layout/alignment.
#[repr(C)]
enum ValueC {
    Integer(u32),
    Float(f32),
    Empty,
}

// repr(C, u8) differs from repr(u8) for data-carrying enums:
// it uses C-style enum layout with a u8 tag representation.
#[repr(C, u8)]
enum SmallTagValue {
    Number(u32),
    Nothing,
}

fn show_enum_sizes() {
    println!("Message size: {}", size_of::<Message>());
    println!("Direction size: {}", size_of::<Direction>());
    println!("ValueC size: {}", size_of::<ValueC>());
    println!("SmallTagValue size: {}", size_of::<SmallTagValue>());

    // Keep variants constructed in this demo so examples are easy to find.
    let _m1 = Message::Quit;
    let _m2 = Message::Move { x: 1, y: 2 };
    let _m3 = Message::Write(String::from("hello"));
    let _direction = Direction::North;
    let _south = Direction::South;
    let _east = Direction::East;
    let _west = Direction::West;
    let _value1 = ValueC::Integer(10);
    let _value2 = ValueC::Float(1.5);
    let _value3 = ValueC::Empty;
    let _small1 = SmallTagValue::Number(42);
    let _small2 = SmallTagValue::Nothing;
}

// ---------------------------------------------------------------------
// 11. UNIONS
// ---------------------------------------------------------------------
//
// A union's fields share the same starting memory location.
//
// Its size is the largest field size rounded up to the union's alignment.
// Its alignment is the maximum alignment of its fields.
//
// Reading a union field is unsafe: you must ensure the chosen field's
// bytes form a valid value of that field's type.
#[repr(C)]
union Number {
    integer: u32,
    bytes: [u8; 4],
}

fn show_union() {
    let number = Number {
        integer: 0x1234_5678,
    };

    println!("Number union size: {}", size_of::<Number>());
    println!("Number union alignment: {}", align_of::<Number>());

    // SAFETY: Reading `integer` is valid because we initialized that field.
    unsafe {
        println!("integer field: {:#x}", number.integer);
    }
}

// ---------------------------------------------------------------------
// 12. #[repr(align(N))]
// ---------------------------------------------------------------------
//
// Raises a type's alignment to at least N. N must be a power of two and
// within the limits documented by Rust. It cannot reduce natural alignment.
//
// This can introduce tail padding because the type's size must be a
// multiple of its alignment.
#[repr(align(16))]
struct Aligned {
    value: u32,
}

// ---------------------------------------------------------------------
// 13. #[repr(packed)] / #[repr(packed(N))]
// ---------------------------------------------------------------------
//
// Reduces the effective alignment used for fields to at most N.
// With repr(C, packed), fields keep declaration order (C repr) but can be
// placed at unaligned offsets.
//
// For this example on the usual target:
// a offset 0, b offset 1, total size 5, alignment 1.
#[repr(C, packed)]
struct Packed {
    a: u8,
    b: u32,
}

fn show_alignment_modifiers() {
    println!(
        "Aligned size/alignment: {}/{}",
        size_of::<Aligned>(),
        align_of::<Aligned>()
    );
    println!(
        "Packed size/alignment: {}/{}",
        size_of::<Packed>(),
        align_of::<Packed>()
    );
    println!("Packed.b offset: {}", std::mem::offset_of!(Packed, b));

    let packed = Packed { a: 1, b: 42 };

    // Do NOT create a normal reference to packed.b:
    // let r = &packed.b; // rejected because the field may be unaligned.
    //
    // Copy the value out, or use a raw pointer and read_unaligned if needed.
    // SAFETY: `addr_of!` creates a raw pointer without creating an unaligned
    // reference. read_unaligned permits reading from an unaligned address.
    let b_value = unsafe { std::ptr::addr_of!(packed.b).read_unaligned() };
    println!("Packed fields: a={}, b={}", packed.a, b_value);
}

// align and packed cannot be combined on the same type.
// A packed type cannot transitively contain a type with stronger alignment
// than the packed type permits.

// ---------------------------------------------------------------------
// 14. #[repr(transparent)]
// ---------------------------------------------------------------------
//
// A transparent wrapper has the same layout and ABI as its one relevant
// non-zero-sized field (or unit if there is no such field).
//
// It can have additional zero-sized fields, such as PhantomData<T>.
// Useful for type-safe wrappers that do not add storage.
#[repr(transparent)]
struct UserId(u64);

#[repr(transparent)]
struct TypedId<T> {
    value: u64,
    marker: PhantomData<T>,
}

fn show_transparent() {
    println!(
        "u64 size/alignment: {}/{}",
        size_of::<u64>(),
        align_of::<u64>()
    );
    println!(
        "UserId size/alignment: {}/{}",
        size_of::<UserId>(),
        align_of::<UserId>()
    );
    println!("TypedId<String> size: {}", size_of::<TypedId<String>>());
}

// ---------------------------------------------------------------------
// 15. VALIDITY RULES: SIZE ALONE IS NOT ENOUGH
// ---------------------------------------------------------------------
//
// bool occupies 1 byte, but valid bool values are only false and true.
// char occupies 4 bytes, but not every u32 bit pattern is a valid char.
// str must contain valid UTF-8.
//
// This matters greatly in unsafe code: allocated bytes fitting a type's
// size does NOT prove those bytes are a valid value of that type.

// ---------------------------------------------------------------------
// 16. LAYOUT VS ABI (preview for later ABI -> FFI study)
// ---------------------------------------------------------------------
//
// Layout: size, alignment, field offsets, representation/discriminants.
// ABI (Application Binary Interface): how compiled code passes/returns
// values and calls functions across a binary boundary.
//
// Same size/alignment does not automatically mean two types have the same
// ABI or are safe to pass between Rust and C.
//
// #[repr(C)] is helpful for C-compatible layout, but does not magically
// make every Rust field type FFI-compatible. Function calling convention,
// valid values, and matching C declarations also matter.

// ---------------------------------------------------------------------
// MAIN
// ---------------------------------------------------------------------

fn main() {
    println!("\n=== Primitive sizes ===");
    show_primitive_sizes();

    println!("\n=== Alignments ===");
    show_alignments();

    println!("\n=== Struct layout and padding ===");
    show_struct_layout_sizes();

    println!("\n=== Sized / ?Sized ===");
    let x: u32 = 42;
    let y: [u8; 10] = [0; 10];
    print_size(x);
    print_size(y);
    inspect(&x);
    inspect("hello"); // T is str, a DST behind &T.

    println!("\n=== Arrays, slices, and strings ===");
    show_arrays_and_slices();

    println!("\n=== Zero-sized types ===");
    show_zero_sized_types();

    println!("\n=== Pointer layouts ===");
    show_pointer_layouts();
    let dog = Dog;
    let animal: &dyn Animal = &dog;
    animal.speak();

    println!("\n=== Tuples and closures ===");
    show_tuple_and_closure_examples();

    println!("\n=== Enums ===");
    show_enum_sizes();

    println!("\n=== Union ===");
    show_union();

    println!("\n=== align / packed ===");
    show_alignment_modifiers();

    println!("\n=== transparent ===");
    show_transparent();

    // Keep these types used in the demo without changing their layout.
    let _default = DefaultRepr { a: 1, b: 2 };
    let _outer = OuterC {
        first: 1,
        inner: InnerDefault { a: 2, b: 3 },
    };
    let _tagged = Tagged::<String> {
        id: 7,
        marker: PhantomData,
    };
    let _user_id = UserId(123);
}
