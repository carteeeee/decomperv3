macro_rules! level_parse_base {
    ($rom:expr, $start:expr) => {{
        let mut iterom = $rom.iter().skip($start).peekable();
        loop {
            let help = iterom.by_ref().next();
            assert!(help != Some(&0x02));
            println!("{:?}", help);
            let next = iterom.peek().unwrap();
            let us = *next.clone() as usize;
            let _ = iterom.by_ref().take(us); // make this advance... somehow
        }
        "".to_owned()
    }};
}

pub fn parse_entry(rom: Vec<u8>, start: usize) -> String {
    level_parse_base!(rom, start)
}
