//
// Reentry
//
// A game by Riskpeep
pub mod rlib;

fn main() {
    //
    // Introduction and Setup
    //
    println!("Welcome to Reentry. A space adventure.");
    println!();
    println!("You awake in darkness with a pounding headache.");
    println!("An alarm is flashing and beeping loudly. This doesn't help your headache.");
    println!();

    let mut command: rlib::Command;
    let mut world = rlib::World::new();
    let mut output: String;

    //
    // Main Loop
    //
    loop {
        command = rlib::get_input();
        output = world.update_state(&command);
        rlib::update_screen(output);

        if matches!(command, rlib::Command::Quit) {
            break;
        }
    }

    //
    // Shutdown and Exit
    //
    println!("Bye!");
}
