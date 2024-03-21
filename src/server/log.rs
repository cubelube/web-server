use std::{
    // open file
    fs::OpenOptions,
    // needed to open in write mode
    io::Write,
    // get system time
    time::SystemTime
};

// extern crate used for formatting time nicely
use chrono::{
    Local, 
    prelude::DateTime
};

fn date_time() -> String
{
    //let datetime_comp = SystemTime::now();
    let datetime_hum = DateTime::<Local>::from(SystemTime::now());
    datetime_hum.format("%Y-%m-%d %H:%M:%S.%f").to_string()
}
// log_type: define type of log
// log_file: where is the log file located?
// connection: in certain logs, shows where connected to
pub fn log(log_type: &str, log_file: &str, connection: &str)
{
    // open log file in write & append mode
    // TODO: error handling here
    let mut log = OpenOptions::new().write(true).append(true).open(log_file).unwrap();

    match log_type
    {
        // log for connecting to server
        "connection" =>
        {
            // format log and write it
            let message = format!("{} [INFO] HANDLING CONNECTION", date_time());
            writeln!(log, "{}", message).unwrap();
        }
        // connecting to /
        "root" =>
        {
            let message = format!("{} [INFO] CONNECTION TO /", date_time());
            writeln!(log, "{}", message).unwrap();
        }
        // connecting elsewhere
        "other" =>
        {
            let message = format!("{} [INFO] CONNECTION TO {}", date_time(), connection);
            writeln!(log, "{}", message).unwrap();
        }
        // connecting somewhere that doesn't exist
        "404" =>
        {
            let message = format!("{} [404] ATTEMPTED CONNECTION TO UNKNOWN {}", date_time(), connection);
            writeln!(log, "{}", message).unwrap();
        }
        // if, for some reason, an invalid log type is passed
        // do nothing
        _ => ()
    };
}
