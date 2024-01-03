# rtic-rp2040

Crate for experimenting with rp2040 0-latency memory accesses.

## Memory layout

The rp2040 has 6 banks of SRAM memory

| Bank  | Origin     | Length |
| ----- | ---------- | ------ |
| SRAM0 | 0x21000000 | 64kB   |
| SRAM1 | 0x21010000 | 64kB   |
| SRAM2 | 0x21020000 | 64kB   |
| SRAM3 | 0x21030000 | 64kB   |
| SRAM4 | 0x20040000 | 4kB    |
| SRAM5 | 0x20041000 | 4kB    |

With a SRAM alias, striping SRAM0, SRAM1, SRAM2, SRAM3.

| Bank | Origin     | Length |
| ---- | ---------- | ------ |
| SRAM | 0x20000000 | 256kB  |

The rp2040 Bus Fabric provides 4 upstreams and 10 downstreams ports, with a total bandwidth 2GByte/s, running at 128MHz (128M \* 4ports \* 4Bytes/transaction).

In order to provide latency free execution instruction fetch, data and stack memory accesses must never access the same upstream port during the same cycle. Notice, the armv6m ISA guarantees that both data and stack accesses will never happen in the same cycle, but to be on the safe side, we dedicate them to separate banks.

The following partitioning guarantees latency free access.

| Core  | Segment | Bank  | Size |
| ----- | ------- | ----- | ---- |
| core0 | code    | SRAM0 | 64kB |
| core0 | data    | SRAM1 | 64kB |
| core0 | stack   | SRAM4 | 4kB  |
| core1 | code    | SRAM2 | 64kB |
| core1 | data    | SRAM3 | 64kB |
| core1 | stack   | SRAM5 | 4kB  |

Notice, all accesses to flash memory will go through the cache, thus might be susceptible to latency due to cache fills, thus we need to ensure that .rodata is mapped to data segment(s).

Peripheral accesses goes over the APB bridge through a single (shared) port and thus cannot be guaranteed latency free access if accessed concurrently.

For the discussion we assume that all peripheral accesses are handled by a single core (e.g., core0), and DMA accesses only affects core1 (i.e., banks SRAM2, SRAM3 and SRAM5 and never touches the peripherals (APB-Bridge)).

Under these restrictions tasks running at core0 should have latency free SRAM access through the rp2040 fabric, and thus operate with fully predictable timing behavior.

We can relax these restrictions by leveraging the two level arbitration scheme and giving core0 priority over core0 DMA. This would allow concurrent access by core0, core1 and DMA to the APB connected peripherals without interference to core0.

## Single core restriction

For our purpose here we restrict to a single core scenario:

| Core  | Segment | Bank  | Size |
| ----- | ------- | ----- | ---- |
| core0 | code    | SRAM0 | 64kB |
| core0 | data    | SRAM1 | 64kB |
| core0 | stack   | SRAM4 | 4kB  |

The provided `memory.x`:

```
MEMORY {
    BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x100
    FLASH : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100
    /* used as core0-data segment, SRAM1 */
    RAM   : ORIGIN = 0x21010000, LENGTH = 64K

    /* used as core0-code segment */
    SRAM0 : ORIGIN = 0x21000000, LENGTH = 64k

    /* used as stack for core0 */
    SRAM4 : ORIGIN = 0x20040000, LENGTH = 4k
}

_stack_start = ORIGIN(SRAM4) + LENGTH(SRAM4);
```

- `ex1.rs`

```rust
static mut RAM_DATA: u32 = 0;

#[link_section = ".core0_code"]
#[inline(never)]
#[no_mangle]
fn ram_function1() {
    unsafe {
        info!("running in ram {}", RAM_DATA);
        RAM_DATA += 1;
    }
}

#[link_section = ".core0_code"]
#[inline(never)]
#[no_mangle]
fn ram_function2() {
    unsafe {
        RAM_DATA -= 1;
        info!("running in ram {}", RAM_DATA);
    }
}
```

To verify our memory layout:

```
cargo objdump --example ex1 --release -- --syms --demangle  > ex1.syms

...
21000000 g     F .core0_code	00000044 ram_function1
21000044 g     F .core0_code	00000044 ram_function2
...
21010038 l     O .bss	00000004 ex1::RAM_DATA::h8e5022c8a5a12ed0 (.0)
...
```

This confirms that we have successfully placed our timing critical code in SRAM0 and allocated static heap variable in SRAM1.

We now run the code with a breakpoint at `ram_function1`, and inspect the registers:

```
R13/SP: 0x20040f88
```

This confirms that we have successfully set the stack pointer to the top of the SRAM4 bank.

Left to assure is that the .rodata is placed in the SRAM1 bank.
