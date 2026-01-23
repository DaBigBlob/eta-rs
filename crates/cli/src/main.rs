use eta_core::runner::runner;

fn main() {
    let mut args = std::env::args();
    let prog = args.next();

    let Some(input) = args.next() else {
        eprintln!("usage: {} S-PAIR", prog.unwrap_or("eta".to_string()));
        std::process::exit(2);
    };

    let mut out = String::new();
    runner(&mut out, &input);
    print!("{out}");
}
