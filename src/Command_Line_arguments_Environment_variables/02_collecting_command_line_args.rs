use std::env::args;

pub fn run(){
    // Process - 01 : collecting command line values
    let command_line_ags1 : Vec<String>= args().collect();
    dbg!(&command_line_ags1);

    // Process - 02 : collecting command line values
    let command_line_ags2 = args().collect::<Vec<String>>();
    dbg!(&command_line_ags2);
}