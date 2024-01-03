use std::{collections::HashMap, fs, time::Duration};

use object::{Object, ObjectSymbol};
use probe_rs::{flashing, Core, MemoryInterface, Permissions, Session};
use symex::{general_assembly::RunConfig, run_elf};

fn main() {
    println!("Utility to measure HW cycles for the rp2040");

    let mut session = Session::auto_attach("rp2040", Permissions::default()).unwrap();

    println!("attached to rp2040 {:?}", session.architecture());

    let path = "test_binarys/calc_crc";

    let hw_measurement = measure_hw(path, &mut session);
    println!("HW measurement: {} cycles", hw_measurement);

    let symex_measurement = measure_symex(path);
    println!("Symex measurement: {} cycles", symex_measurement);
}

fn measure_symex(path: &str) -> u64 {
    let cfg = RunConfig {
        show_path_results: false,
        pc_hooks: vec![],
        register_read_hooks: vec![],
        register_write_hooks: vec![],
        memory_write_hooks: vec![],
        memory_read_hooks: vec![],
    };
    let results = run_elf::run_elf(path, "measure", cfg).unwrap();
    let mut max = 0;

    for result in results {
        max = max.max(result.max_cycles);
    }

    max as u64
}

fn measure_hw(path: &str, session: &mut Session) -> u64 {
    flashing::download_file(session, path, flashing::Format::Elf).unwrap();
    let mut core = session.core(0).unwrap();

    // Setup for measurement
    core.halt(Duration::from_millis(500)).unwrap();
    core.clear_all_hw_breakpoints().unwrap();

    // Start program
    core.reset().unwrap();

    // Wait until first measuring point
    core.wait_for_core_halted(Duration::from_millis(500))
        .unwrap();
    let start = core.read_word_32(0xe000e018).unwrap() & 0x00FFFFFF;

    // run until next measuring point
    core.run().unwrap();
    core.wait_for_core_halted(Duration::from_millis(500))
        .unwrap();

    let end = core.read_word_32(0xe000e018).unwrap() & 0x00FFFFFF;

    // calculate a measured time
    // compensate for bkpt discrepancy by subtracting 6 (determined by experimentation)
    let diff = start - end - 6;
    diff as u64
}
