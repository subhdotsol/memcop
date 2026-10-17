# memcop

**A Rust systems inspection CLI for exploring type layouts, memory allocations, and Linux process memory.**

`memcop` is a command-line tool built to make low-level memory concepts observable. It lets you inspect how Rust types are laid out, experiment with stack values and heap-backed collections, examine a Linux process's virtual memory mappings, and read memory statistics exposed by the operating system.

The project is also a hands-on way to learn Rust systems programming, Linux internals, memory management, and performance investigation.

> **Status:** In development. Commands and features described below define the intended interface. They should not be considered implemented until they are built and tested.

## Goals

- Inspect the size and alignment of Rust types.
- Understand struct layout, field offsets, and padding.
- Observe the behaviour of `Box<T>`, `Vec<T>`, and `String`.
- Compare allocation capacity with logical data length.
- Inspect Linux virtual memory mappings through `/proc`.
- Read process memory statistics such as virtual size and resident memory.
- Run repeatable experiments and compare observations.
- Build a small, well-tested systems utility using Rust's standard library.

## Commands

The intended command-line interface is:

```text
memcop <COMMAND> [OPTIONS]
```

Run `memcop --help` to see available commands and `memcop <COMMAND> --help` for command-specific options.

| Command | Purpose |
|---|---|
| `memcop layout` | Inspect the size, alignment, and layout of supported Rust types. |
| `memcop alloc` | Run experiments with stack values, boxes, vectors, and strings. |
| `memcop maps` | Inspect the current process's Linux virtual memory mappings. |
| `memcop status` | Read memory-related metrics from Linux `/proc` process information. |
| `memcop report` | Combine layout, allocation, and process-memory observations into one report. |

The Linux-specific commands require a Linux environment with a mounted `/proc` filesystem. The layout and allocation experiments can run on other supported platforms, but their observations may differ by target.

---

## 1. `memcop layout`

Inspect the memory layout of supported Rust types.

```bash
memcop layout
memcop layout --type u8
memcop layout --type u64
memcop layout --type Example
```

### What it does

For each supported type, display:

- Type name
- Size in bytes
- Alignment in bytes
- Field names and field offsets, where registered field information is available
- Total struct size and padding information, where it can be calculated reliably

Example output:

```text
memcop | Type Layout

Primitive Types
---------------
u8       size: 1 byte    alignment: 1 byte
u64      size: 8 bytes   alignment: target-dependent
usize    size: target-dependent
&u64     size: target-dependent

Structs
------
Example
  size:      <measured bytes>
  alignment: <measured bytes>

  Field       Offset
  -----       ------
  a           <measured bytes>
  b           <measured bytes>
  c           <measured bytes>
```

Values shown as target-dependent are measured on the system where the program runs. The actual output will contain the measured values.

### Concepts covered

- `std::mem::size_of`
- `std::mem::align_of`
- `std::mem::offset_of!`
- Primitive types and pointer-sized types
- Struct padding and alignment
- `#[repr(Rust)]` versus `#[repr(C)]`
- Target architecture and ABI differences

### Important limitation

Rust does not provide a general-purpose runtime API that can inspect every arbitrary type in every compiled binary.

`memcop` will inspect built-in examples and types explicitly registered in its own source code. It will not automatically discover every type in another crate or executable.

Field offsets and padding will be reported only when the program has the required layout information. For default Rust representation, field ordering is not generally guaranteed to match source order.

---

## 2. `memcop alloc`

Run experiments that demonstrate stack values, heap-backed data structures, pointers, lengths, and capacities.

```bash
memcop alloc
memcop alloc --type box
memcop alloc --type vec
memcop alloc --type string
memcop alloc --type all
```

### What it does

The command will demonstrate:

- A local value and its address
- A `Box<T>` and its pointee address
- A `Vec<T>`'s length, capacity, and element-buffer pointer
- A `String`'s length, capacity, and buffer pointer
- How collection capacity changes as elements are added
- The difference between an object and the data it refers to

Example output:

```text
memcop | Allocation Experiments

Stack Value
-----------
Type:      u64
Value:     42
Size:      8 bytes
Address:   <runtime address>

Box<T>
------
Value:          42
Box handle size: <measured bytes>
Pointee address: <runtime address>

Vec<T>
------
Length:          3
Capacity:        4
Element size:    8 bytes
Buffer address:  <runtime address>

String
------
Length:          11 bytes
Capacity:        <runtime capacity>
Buffer address:  <runtime address>
```

The values and addresses above are illustrative placeholders, not fixed output.

### Concepts covered

- Stack and heap models
- Ownership and borrowing
- Pointers and references
- `Box<T>`
- `Vec<T>` and `String`
- Length versus capacity
- Heap allocation and reallocation
- Why an address alone does not tell you how memory is managed

### Important limitation

A variable's address does not prove that it resides on a particular physical memory location. Compiler optimisations, temporary values, and implementation details can affect what you observe.

Likewise, the `Vec<T>` or `String` object is distinct from its backing buffer. The command will report observable addresses and metadata rather than claiming to show every allocation performed by the process.

---

## 3. `memcop maps`

Inspect the virtual memory mappings of the running `memcop` process.

```bash
memcop maps
memcop maps --filter heap
memcop maps --filter stack
memcop maps --filter executable
```

**Platform:** Linux.

### What it does

Read `/proc/self/maps` and parse the process's memory mappings.

Where available, display:

- Start and end virtual addresses
- Mapping size
- Read, write, and execute permissions
- Private or shared mapping indicator
- File offset
- Device and inode information
- Associated pathname, when present

Example output:

```text
memcop | Process Memory Maps

START-END           PERMS  SIZE       PATH
------------------  -----  ---------  ----------------
<address range>     r-xp   <size>     executable
<address range>     rw-p   <size>     anonymous mapping
<address range>     rw-p   <size>     [heap]
<address range>     rw-p   <size>     [stack]
```

Actual mappings depend on the process, operating system, runtime, and environment. Not every process will have every named mapping.

### Concepts covered

- Virtual address spaces
- Memory mappings
- Mapping permissions
- Anonymous versus file-backed mappings
- Shared versus private mappings
- Stack and heap concepts
- Address Space Layout Randomisation (ASLR)
- The Linux `/proc` filesystem

### Important limitation

`/proc/self/maps` describes the virtual mappings of the process reading the file. It does not directly enumerate every Rust allocation or show physical RAM usage.

A mapping labelled `[heap]` is not a complete inventory of heap memory. Allocators can obtain memory through different mechanisms, including anonymous mappings.

---

## 4. `memcop status`

Inspect memory statistics exposed by Linux for the running process.

```bash
memcop status
```

**Platform:** Linux.

### What it does

Read and parse `/proc/self/status`.

The initial version will focus on available fields such as:

- `VmSize`: virtual memory size
- `VmRSS`: resident set size
- `VmHWM`: peak resident set size
- `VmData`: data segment size
- `VmStk`: stack size reported by the kernel
- `VmExe`: executable code size
- `VmLib`: shared library code size

Some fields may be absent or have different applicability depending on the kernel and process.

Example output:

```text
memcop | Process Memory Status

Virtual Memory
--------------
VmSize: <reported value>

Resident Memory
---------------
VmRSS:  <reported value>
VmHWM:  <reported value>

Other Metrics
-------------
VmData: <reported value>
VmStk:  <reported value>
VmExe:  <reported value>
VmLib:  <reported value>
```

### Concepts covered

- Virtual memory size versus resident memory
- Resident Set Size (RSS)
- Peak resident memory
- Demand paging
- Why allocated memory and resident memory are different
- Kernel-reported process statistics
- Memory measurement limitations

### Important limitation

These values are observations reported by Linux, not exact accounting of every allocation. In particular, `VmRSS` and related fields can be inaccurate or subject to kernel accounting limitations.

`memcop status` will display the values reported by the operating system without presenting them as perfect measurements of actual application memory consumption.

---

## 5. `memcop report`

Combine the tool's supported experiments into a single report.

```bash
memcop report
```

### What it does

The intended report brings together:

1. Type layout observations
2. Allocation experiment results
3. Linux memory mapping information
4. Linux process memory statistics

Example structure:

```text
memcop | Memory Report

[1] Type Layout
    Primitive type sizes and alignments
    Registered struct layouts

[2] Allocation Experiments
    Box<T>
    Vec<T>
    String

[3] Virtual Memory Maps
    Mapping count
    Mapping ranges and permissions

[4] Process Memory Status
    VmSize
    VmRSS
    VmHWM

[5] Notes
    Platform
    Architecture
    Measurement limitations
```

The report should clearly distinguish measured values from explanatory notes and unavailable metrics.

On non-Linux platforms, Linux-specific sections will be reported as unsupported rather than fabricated.

---

## Design principles

### 1. Prefer the standard library

The initial implementation should use Rust's standard library wherever practical.

The goal is to understand the underlying concepts before adding dependencies.

### 2. Safe Rust first

Use safe Rust for parsing, data modelling, command handling, and reporting.

Introduce `unsafe` only when a specific experiment requires it, and explain the safety requirements in the code and documentation.

### 3. Measure rather than assume

Results such as alignment, pointer values, collection capacity, memory mappings, and process statistics must be measured at runtime where appropriate.

Do not hard-code platform-specific observations as universal facts.

### 4. Keep operating-system behaviour separate

The type-layout and allocation experiments should be separated from Linux-specific `/proc` functionality.

This keeps the project easier to test and makes unsupported platform features explicit.

### 5. Test the parser independently

Linux `/proc` files have structured formats that should be parsed into Rust data types.

Parsing should be separate from printing, allowing unit tests to use representative input strings without depending on a live operating system.

### 6. Explain the limitations

Memory inspection is not the same as complete memory tracing. The documentation should distinguish type layout, virtual memory mappings, allocator behaviour, and kernel memory statistics.

---

## Planned project structure

```text
memcop/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs
    ├── cli.rs
    ├── layout.rs
    ├── allocations.rs
    ├── report.rs
    └── linux/
        ├── mod.rs
        ├── maps.rs
        └── status.rs
```

### Module responsibilities

| Module | Responsibility |
|---|---|
| `main.rs` | Application entry point |
| `cli.rs` | Command-line parsing and dispatch |
| `layout.rs` | Type size, alignment, and registered field-layout experiments |
| `allocations.rs` | Stack-value, `Box`, `Vec`, and `String` experiments |
| `report.rs` | Combine results into a readable report |
| `linux/maps.rs` | Parse Linux process memory mappings |
| `linux/status.rs` | Parse Linux process memory statistics |
| `linux/mod.rs` | Organise Linux-specific functionality |

The exact module boundaries may evolve as the implementation develops.

---

## Development roadmap

### Phase 1: Type layout

- [ ] Build the CLI entry point.
- [ ] Add `memcop layout`.
- [ ] Measure primitive type sizes and alignments.
- [ ] Add reference and pointer-sized type experiments.
- [ ] Define example structs.
- [ ] Inspect field offsets and padding.
- [ ] Compare `#[repr(Rust)]` and `#[repr(C)]`.
- [ ] Add unit tests.

### Phase 2: Allocation experiments

- [ ] Add `memcop alloc`.
- [ ] Inspect local values and addresses.
- [ ] Inspect `Box<T>`.
- [ ] Inspect `Vec<T>` length and capacity.
- [ ] Inspect `String` length and capacity.
- [ ] Demonstrate collection growth.
- [ ] Document what addresses and metadata do not prove.

### Phase 3: Linux memory inspection

- [ ] Add Linux platform detection.
- [ ] Parse `/proc/self/maps`.
- [ ] Display virtual memory mappings and permissions.
- [ ] Add mapping filters.
- [ ] Parse `/proc/self/status`.
- [ ] Display memory statistics with units.
- [ ] Test parsers against representative fixtures.
- [ ] Handle missing files and malformed input gracefully.

### Phase 4: Reports and quality

- [ ] Add `memcop report`.
- [ ] Add clear errors and exit codes.
- [ ] Add command-specific help.
- [ ] Run `cargo fmt --check`.
- [ ] Run `cargo test`.
- [ ] Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [ ] Document supported platforms and limitations.
- [ ] Record example output from a known environment.

---

## Build and run

You will need a working Rust toolchain and Cargo.

Clone the repository once it has been created:

```bash
git clone <repository-url>
cd memcop
```

Build the project:

```bash
cargo build
```

Run the commands during development:

```bash
cargo run -- layout
cargo run -- alloc
cargo run -- maps
cargo run -- status
cargo run -- report
```

Run the test suite:

```bash
cargo test
```

Run formatting and lint checks:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

These commands describe the intended workflow; they will work as documented once the corresponding CLI commands and options have been implemented.

For Linux-specific features, use a Linux environment with `/proc` available. On a macOS development machine, you can build and test the cross-platform parts locally and run the Linux experiments inside a Linux virtual machine.

---

## Learning resources

These are the primary references for the concepts explored by `memcop`.

### Rust type layout

- [Rust Reference: Type Layout](https://doc.rust-lang.org/reference/type-layout.html)
- [`size_of`](https://doc.rust-lang.org/std/mem/fn.size_of.html)
- [`align_of`](https://doc.rust-lang.org/std/mem/fn.align_of.html)
- [`offset_of!`](https://doc.rust-lang.org/std/mem/macro.offset_of.html)

### Ownership and allocation

- [The Rust Book: Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [`Box<T>`](https://doc.rust-lang.org/std/boxed/struct.Box.html)
- [`Vec<T>`](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
- [`std::alloc`](https://doc.rust-lang.org/std/alloc/index.html)

### Linux memory

- [`proc_pid_maps(5)`](https://man7.org/linux/man-pages/man5/proc_pid_maps.5.html)
- [`proc_pid_status(5)`](https://man7.org/linux/man-pages/man5/proc_pid_status.5.html)
- [`proc_pid_statm(5)`](https://man7.org/linux/man-pages/man5/proc_pid_statm.5.html)
- [`mmap(2)`](https://man7.org/linux/man-pages/man2/mmap.2.html)
- [Linux kernel memory-management concepts](https://docs.kernel.org/admin-guide/mm/concepts.html)

### Testing and project structure

- [The Rust Book: Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [The Rust Book: Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [The Rust Book: Tests](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Clippy](https://doc.rust-lang.org/clippy/)

---

## Scope and non-goals

`memcop` is intended to be an educational systems-inspection CLI, not a replacement for a full profiler, debugger, allocator tracer, or kernel observability tool.

The initial scope does **not** include:

- Inspecting arbitrary types from any compiled binary
- Reading or modifying another process's memory
- Tracking every heap allocation in a program
- Reporting exact physical RAM consumption for every allocation
- Providing a complete memory leak detector
- Guaranteeing identical layouts or output across architectures and compiler versions

These may be explored later if they have a clear use case and can be implemented with well-defined safety and correctness guarantees.

## Long-term direction

Once the core commands are reliable, possible extensions include:

- Allocation-growth experiments with configurable sizes
- Before-and-after process-memory measurements
- Structured output such as JSON
- Repeated measurements for comparing runs
- Optional allocator instrumentation
- More detailed virtual-memory analysis
- Benchmarks for selected allocation patterns
- Cross-platform memory information where reliable platform APIs exist

Extensions should be added incrementally, with tests and documentation for each feature.

## Project philosophy

`memcop` should make systems concepts observable, explain what the observations mean, and be honest about what they cannot tell us.

The aim is not to print memory addresses for their own sake. It is to connect Rust's type system and allocation abstractions to the behaviour of a running process and the operating system beneath it.
