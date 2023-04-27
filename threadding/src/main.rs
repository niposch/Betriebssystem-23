use std::{process, env::args, time::Instant};

use nix::libc;


fn work(process_name: &str, log_progress: bool){
    const PRINT_INTERVAL: u32 = 10_000_000;
    for i in 0..u32::MAX {
        if log_progress && i % PRINT_INTERVAL == 0 {
            println!("{}: {}%", process_name, 100.0 * i as f32 / u32::MAX as f32);
        }
    }
}

// run via `taskset -c 1 cargo run 1 true` (number from 1 to 19) (omit true to disable most of the logging)
fn main() {
    // if it isn't running under linux, exit
    if cfg!(not(target_os = "linux")) {
        eprintln!("This program only runs on linux");
        process::exit(1);
    }
    // get priority from args
    let priority = args().nth(1)
        .expect("Please provide a priority")
        .parse::<i32>() // parse as number
        .expect("Please provide a number")
        .clamp(1, 19); // limit between 1 and 19

    let log_progress = args().nth(2)
        .map(|arg| arg == "true")
        .unwrap_or(false);
        

    // create a child process
    match unsafe { libc::fork() }{
        0 => {
            // child process
            let start = Instant::now();
            let actual_priority = unsafe { libc::getpriority(libc::PRIO_PROCESS, std::process::id()) };
            println!("child process priority wasnt modified, current priority is {}", actual_priority);
            work("child", log_progress);
            print!("child process finished");
            print!("Time elapsed child thread is {:?}", start.elapsed());
        },
        pid if pid > 0 => {
            // parent process

            // set priority
            unsafe { libc::setpriority(libc::PRIO_PROCESS, std::process::id(), priority) };
            let actual_priority = unsafe { libc::getpriority(libc::PRIO_PROCESS, std::process::id()) };
            println!("parent process was set to priority {} (actual Priority {})", priority, actual_priority);

            // start a stopwatch
            let start = Instant::now();
            
            work("parent", log_progress);

            // stop stopwatch
            let duration = start.elapsed();

            print!("parent process finished");
            println!("Time elapsed parent thread is {:?}", duration);
        },

        _ => {
            // error
            eprintln!("error");
            process::exit(1);
        },
    }
}
