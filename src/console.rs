use std::{
    // open files
    fs::{self, File, OpenOptions},
    io::{
        // get user input
        stdin,
        // flush stdout 
        stdout, 
        Write
    }, 
    // terminal commands and exit function
    process::{exit, Command}
};

// make terminal output pretty
use colored::Colorize;

// enum tricks for match function
enum MatchReturns
{
    Help(()),
    Log(i32),
    Clear(()),
    History(u32),
    None
}

// log_file: log file path
// name: name of server (to be printed at console)
pub fn console(log_file: &str, name: &str, backup: &str, history_path: &str)
{
    loop
    {
        let mut console_input = String::new();
        print!("{} > ", name);
        stdout().flush().unwrap();
        stdin().read_line(&mut console_input).unwrap();

        // have to trim because read_line() appends \n
        match console_input.to_lowercase().as_str().trim()
        {
            "help" | "?" => MatchReturns::Help(help(history_path)),
            "exit" => exit(0),
            "log" => MatchReturns::Log(console_log(log_file, backup, history_path)),
            "clear" => MatchReturns::Clear(clear_screen(history_path)),
            "history" => MatchReturns::History(history(history_path)),
            "history clear" => MatchReturns::History(history_clear(history_path)),
            _ => MatchReturns::None,
        };
    }
}

fn history_clear(path: &str) -> u32
{
    File::create(path).unwrap();
    0
}

fn history(path: &str) -> u32
{
    let mut file_history = OpenOptions::new().write(true).append(true).open(path).unwrap();
    writeln!(file_history, "history").unwrap();
    let history_file = fs::read_to_string(path).unwrap();

    let mut line_counter = 1;

    for line in history_file.lines()
    {
        println!("{}. {}", line_counter, line);
        line_counter += 1;
    }

    0
}

fn help(path: &str)
{
    let mut file_history = OpenOptions::new().write(true).append(true).open(path).unwrap();
    writeln!(file_history, "clear").unwrap();

    println!("\n{}: Prints help page. Also available as {}", "?".yellow().bold(), "help".yellow().bold());
    println!("{}: Forwards you to log console. Additional instructions available there.", "log".yellow().bold());
    println!("{}: Clears the screen.", "clear".yellow().bold());
    println!("{}: Immediately shuts down the server. Careful!\n", "exit".yellow().bold());
}

fn clear_screen(path: &str)
{
    let mut file_history = OpenOptions::new().write(true).append(true).open(path).unwrap();
    // if windows, execute cls
    if cfg!(target_os = "windows")
    {
        writeln!(file_history, "cls").unwrap();
        Command::new("cls").status().unwrap();
    }
    // if linux, execute clear
    else if cfg!(target_os = "linux")
    {
        writeln!(file_history, "clear").unwrap();
        Command::new("clear").status().unwrap();
    }
    // otherwise, can't clear
    // BSD etc. would probably work with clear also
    // so TODO: change it to target_family instead of target_os?
    else
    {
        println!("Unable to clear screen - unknown OS");
    }
}

fn console_log(file_path: &str, backup_path: &str, history: &str) -> i32
{
    let mut file_history = OpenOptions::new().write(true).append(true).open(history).unwrap();
    writeln!(file_history, "log").unwrap();
    loop
    {
        let mut choice = String::new();
        println!("\n{}. Clear log file\n{}. Print log file to terminal\n{}. Back up log file\n{}. Return to console", 
        "1".yellow().bold(), "2".yellow().bold(), "3".yellow().bold(), "99".yellow().bold());
        print!("log > ");
        stdout().flush().unwrap();
        stdin().read_line(&mut choice).unwrap();

        // TODO: better error handling
        match choice.trim().parse::<i32>()
        {
            Ok(_) => (),

            Err(e) =>
            {
                
                println!("Unable to parse input: {}", e);
                println!("Returning to console. Please try again.");

                return 1
            }
        }

        let choice1: i32 = choice.trim().parse().unwrap();

        if choice1 == 1
        {
            loop {
                let mut second_choice = String::new();
                print!("Are you sure you want to clear the log file? y/n ");
                stdout().flush().unwrap();
                stdin().read_line(&mut second_choice).unwrap();

                match second_choice.trim()
                {
                    "y" => 
                        {
                            writeln!(file_history, "log -> clear").unwrap();
                            match fs::read_to_string(backup_path)
                            {
                                Ok(_) =>
                                {
                                    let mut backup_file = OpenOptions::new().write(true).append(true).open(backup_path).unwrap();
                                    for line in fs::read_to_string(file_path).unwrap().lines()
                                    {
                                        writeln!(&mut backup_file, "{}", line).unwrap();
                                    }
                                    File::create(file_path).unwrap();
                                    break
                                }
                                Err(_) => panic!("fix this later"),
                            }
                        }
                    "n" => break,
                    _ => (),
                };
            }
        }
        else if choice1 == 2
        {
            writeln!(file_history, "log -> print").unwrap();
            loop
            {
                let mut decide = String::new();

                println!("{}. Print entire file\n{}. Print 404 errors\n{}. Print warnings\n{}. Go back", 
                "1".yellow().bold(), "2".yellow().bold(), "3".yellow().bold(), "99".yellow().bold());
                print!("log - print > ");
                stdout().flush().unwrap();
                stdin().read_line(&mut decide).unwrap();

                let decide1: i32 = decide.trim().parse().unwrap();

                if decide1 == 1
                {
                    // TODO same as above cfg! in clear_screen()
                    if cfg!(target_os = "windows")
                    {
                        writeln!(file_history, "log -> print -> print").unwrap();
                        Command::new("print").arg(file_path).status().unwrap();
                    }
                    else if cfg!(target_os = "linux")
                    {
                        writeln!(file_history, "log -> print -> cat").unwrap();
                        Command::new("cat").arg(file_path).status().unwrap();   
                    }
                    else
                    {
                        println!("Unable to print log file - unknown OS");    
                    }

                    println!("");
                }
                else if decide1 == 2
                {
                    if cfg!(target_os = "linux")
                    {
                        writeln!(file_history, "log -> print -> [404]").unwrap();
                        // TODO same as above but add windows support
                        println!("");
                        Command::new("grep").arg("-F").arg("[404]").arg(file_path).status().unwrap();
                        println!("");
                    }
                    else
                    {
                        println!("Unable to print 404 errors - unknown OS");    
                    }
                }
                else if decide1 == 3
                {
                    // TODO ditto
                    if cfg!(target_os = "linux")
                    {
                        writeln!(file_history, "log -> print -> [WARN]").unwrap();
                        Command::new("grep").arg("-F").arg("[WARN]").arg(file_path).status().unwrap();
                    }
                    else
                    {
                        println!("Unable to print warnings - unknown OS");    
                    }
                }
                else if decide1 == 99
                {
                    break
                }
            }
        }
        else if choice1 == 99
        {
            return 0
        }
    }
}
