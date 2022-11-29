//
// Reentry
//
// A game by Riskpeep
pub mod rlib;

// Game file location
const GAME_FILE_LOC: &str = "./game_file.ron";

fn main() {
    let world_res = init_game(GAME_FILE_LOC);

    match world_res {
        Ok(world) => {
            //
            // Run Game
            do_game(world);
        }
        Err(file_err) => {
            //
            // Shutdown and exit with error
            println!("ERROR - {}", file_err);
        }
    }
}

fn init_game(file_loc: &str) -> Result<rlib::World, std::io::Error> {
    // Read the game file and return the returned world.
    // Bubble up any error result
    rlib::World::read_from_file(file_loc)
}

fn do_game(mut world: rlib::World) {
    let mut command: rlib::Command;
    let mut output: String;

    //
    // Introduction and Setup
    //
    println!("Welcome to Reentry. A space adventure.");
    println!();
    println!("You awake in darkness with a pounding headache.");
    println!("An alarm is flashing and beeping loudly. This doesn't help your headache.");
    println!();

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
