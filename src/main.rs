mod engine;

const DEFAULT_WIDTH_WINDOW: u32 = 500;
const DEFAULT_HEIGHT_WINDOW: u32 = 500;
const DEFAULT_WIDTH_WORLD: usize = 100;
const DEFAULT_HEIGHT_WORLD: usize = 100;
const DEFAULT_TICK_SPEED: f64 = 60.0;

fn main() -> Result<(), i32> {
    let mut win_width: u32 = DEFAULT_WIDTH_WINDOW;
    let mut win_height: u32 = DEFAULT_HEIGHT_WINDOW;
    let mut world_width: usize = DEFAULT_WIDTH_WORLD;
    let mut world_height: usize = DEFAULT_HEIGHT_WORLD;
    let mut tick_speed: f64 = DEFAULT_TICK_SPEED;

    let argv: Vec<String> = std::env::args().collect();
    let argc = argv.len();
    let mut i = 1;
    while i < argc { //Skip the first one, it's the executable itself
        let arg = &argv[i];
        match arg.as_str() {
            "--help" | "-h" => {
                print_help(HelpType::General);
                return Ok(());
            }
            "--window" | "-w" => {
                if i + 2 >= argv.len() {
                    print_help(HelpType::Window);
                    return Err(1);
                }
                win_width = match argv[i + 1].parse::<u32>() {
                    Ok(w) => w,
                    Err(_) => {
                        print_help(HelpType::Window);
                        return Err(1);
                    }
                };
                win_height = match argv[i + 2].parse::<u32>() {
                    Ok(h) => h,
                    Err(_) => {
                        print_help(HelpType::Window);
                        return Err(1);
                    }
                };
                i += 3;
            }
            "--size" | "-s" => {
                // Get the two next arguments, and skip them
                if i + 2 >= argv.len() {
                    print_help(HelpType::Size);
                    return Err(1);
                }
                world_width = match argv[i + 1].parse::<usize>() {
                    Ok(w) => w,
                    Err(_) => {
                        print_help(HelpType::Size);
                        return Err(1);
                    }
                };
                world_height = match argv[i + 2].parse::<usize>() {
                    Ok(h) => h,
                    Err(_) => {
                        print_help(HelpType::Size);
                        return Err(1);
                    }
                };
                i += 3;
            }
            "--tickspeed" | "-t" => {
                if i + 1 >= argv.len() {
                    print_help(HelpType::TickSpeed);
                    return Err(1);
                }
                tick_speed = match argv[i + 1].parse::<f64>() {
                    Ok(t) => t,
                    Err(_) => {
                        print_help(HelpType::TickSpeed);
                        return Err(1);
                    }
                };
                i += 2;
            }
            s => {
                print_help(HelpType::Invalid(s));
                return Ok(());
            },
        }
    }

    println!("Running game with the following parameters:\n\
    Window size: {win_width}x{win_height}\n\
    World size: {world_width}x{world_height}\n\
    Tick speed: {tick_speed}");

    let mut game = engine::game::Game::new((world_width, world_height), (win_width, win_height));
    game.game_loop(tick_speed);

    Ok(())
}

enum HelpType<'a> {
    General,
    Invalid(&'a str),
    Window,
    Size,
    TickSpeed,
}

fn print_help(help: HelpType) {
    match help {
        HelpType::General => {
            println!("Welcome to Pokemon Game of Life !\n\
            ==-==-==-==-==-==-==-==-==-==-==-==-==-==-==-==-==-==-\n\
            --help -> Prints this message.\n\
            --window x y -> Sets the width and height of the window, respectively.\n\
            --size x y -> Sets the width and height of the world, respectively.\n\
            --tickspeed x -> Sets the speed of the game, the higher is x, the faster is the game. |CAREFUL: BIG VALUES WILL BREAK|")
        }
        HelpType::Invalid(s) => {
            println!(
                "Invalid argument passed ! {s}\n\
            Type --help for more usage information."
            );
        }
        HelpType::Window => {
            println!(
                "You must input two valid numbers!\n\
            Example: --window 1000 1500"
            );
        }
        HelpType::Size => {
            println!(
                "You must input two valid numbers!\n\
            Example: --size 100 50"
            );
        }
        HelpType::TickSpeed => {
            println!(
                "You must input a valid non zero number! (big numbers will break the program)\n\
            Example: --tickspeed 60"
            );
        }
    }
}
