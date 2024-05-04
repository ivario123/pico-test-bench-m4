use std::{collections::HashMap, fmt::Display, fs, time::Duration};

use object::{Object, ObjectSymbol};
use probe_rs::{
    architecture::arm::Register, flashing, Core, MemoryInterface, Permissions, Session,
};
use symex::{general_assembly::RunConfig, run_elf};

use std::fs::File;
use std::io::prelude::*;

struct Measurement {
    name: String,
    hw: u64,
    symex: u64,
}

impl Display for Measurement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\t{}\t{}", self.name, self.hw, self.symex)
    }
}

fn main() {
    println!("Utility to measure HW cycles for the nRF52840_xxAA");

    let mut session = Session::auto_attach("nRF52840_xxAA", Permissions::default()).unwrap();

    println!("attached to nRF52840_xxAA {:?}", session.architecture());

    println!("name\t\thw\tsymex");
    for to_test in fs::read_dir("test_binarys").unwrap() {
        // if to_test
        // .as_ref()
        //     .is_ok_and(|el| el.file_name().to_str().clone().unwrap() == "nop_loop")
        // {
        let path = to_test.unwrap().path();
        let path_str = path.to_string_lossy().to_string();
        let name = path_str.split('/').last().unwrap();
        println!("Measuring : {name}");
        let hw_measurement = measure_hw(&path_str, &mut session);
        println!("HW : {hw_measurement}");

        let symex_measurement = measure_symex(&path_str);

        // let measurement = Measurement {
        //     name: name.to_owned(),
        //     hw: hw_measurement,
        //     symex: symex_measurement,
        // };
        //
        let mut f = File::options()
            .create(true)
            .append(true)
            .open("exec.log")
            .unwrap();
        write!(
            &mut f,
            "Name : {name} \n\thw \t: {hw_measurement} \n\tsymex \t: {symex_measurement}\n"
        );
        println!("{name} Done");
        // }
    }
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
    // core.reset();
    // // Setup for measurement
    // core.halt(Duration::from_millis(500)).unwrap();
    // let end_val: u32 = core.read_core_reg(15).unwrap();
    // let mut started = false;
    // for i in 0..6000 {
    //     core.step();
    //     let val: u32 = core.read_core_reg(15).unwrap();
    //     if val < end_val {
    //         started = true;
    //     }
    //     if started && val > end_val {
    //         println!("BACK IN MAIN");
    //         break;
    //     }
    //     println!("PC : {:#04x}", val);
    // }
    core.reset();

    core.halt(Duration::from_millis(500)).unwrap();
    core.clear_all_hw_breakpoints().unwrap();

    // Start program
    core.reset().unwrap();

    // Wait until first measuring point
    core.wait_for_core_halted(Duration::from_millis(5000))
        .unwrap();
    let start = core.read_word_32(0xe000e018).unwrap() & 0x00FFFFFF;

    // run until next measuring point
    core.run().unwrap();
    core.wait_for_core_halted(Duration::from_millis(500))
        .unwrap();

    let end = core.read_word_32(0xe000e018).unwrap() & 0x00FFFFFF;

    // calculate a measured time
    // compensate for bkpt discrepancy by subtracting 6 (determined by experimentation)
    let diff = start - end - 5;
    diff as u64
}
