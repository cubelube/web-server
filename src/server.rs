mod log;

use std::{
    // read/create files
    fs,
    
    // read incoming stream, parse errors, etc.
    io::{prelude::*, BufReader, ErrorKind},

    // handle TCP connections
    net::TcpStream,
};

// custom log function
use log::log;

pub fn handle_connection(mut stream: TcpStream)
{
    // read incoming stream
    let buf_reader = BufReader::new(&mut stream);
    let request = buf_reader.lines().next().unwrap().unwrap();
    let file = request.split_whitespace().nth(1).unwrap();

    //get html directory
    let dir = parse_config("ROOT_DIR");

    // get log file path
    let log_file = parse_config("LOG_FILE");

    // get html file name (usually index.html)
    let file_name = parse_config("ROOT_FILE");

    // get 404 file name
    let not_found = parse_config("404_file");
    
    // log incoming connection
    log("connection", &log_file, "");

    match file
    {
        // if going to /
        "/" =>
        {
            // log connection to /
            log("root", &log_file, "/");

            // get html path and read to string
            let root_path = format!("{}/{}", dir, file_name);
            let html = fs::read_to_string(root_path.clone());

            // if html exists
            match html
            {
                Ok(_) =>
                {
                    // read html to string and send it to recipient
                    let status_line = "HTTP/1.1 200 OK\r\n\r\n";
                    let html = html.unwrap();
                    let response = format!("{status_line}\r\n\r\n{html}");
                    stream.write_all(response.as_bytes()).unwrap();
                },

                // otherwise
                Err(error) =>
                {
                    match error.kind()
                    {
                        // if requested file is not found
                        ErrorKind::NotFound =>
                        {
                            // create requested file
                            let index_path = format!("{}/{}", parse_config("ROOT_DIR"), file_name);
                            println!("{}", index_path);

                            // TODO: better error handling? (maybe not necessary)
                            fs::File::create(index_path).unwrap();
                        },
                        // other error
                        _ =>
                        {
                            // immediately panic program
                            // TODO: find other ways to deal with this?
                            // I feel like immediately panicking is not the best option
                            panic!("unexpected error with index file")
                        }
                    }

                    // now that you've created file, send to recipient
                    let status_line = "HTTP/1.1 200 OK\r\n\r\n";
                    let html = fs::read_to_string(root_path).unwrap();
                    let response = format!("{status_line}\r\n\r\n{html}");
                    stream.write_all(response.as_bytes()).unwrap();
                }
            }
            
        }

        // if going somewhere else
        _ =>
        {
            // get html file path and read it to string
            let file_path = format!("{}{}/{}", dir, file, file_name);
            let html = fs::read_to_string(file_path.clone());

            match html
            {
                Ok(_) =>
                {
                    // log successful connection
                    log("other", &log_file, file_path.as_str());
                    let status_line = "HTTP/1.1 200 OK\r\n\r\n";
                    let html = html.unwrap();
                    let response = format!("{status_line}\r\n\r\n{html}");

                    // send to recipient
                    stream.write_all(response.as_bytes()).unwrap();
                },
                // if html does not exist
                Err(_) =>
                {
                    // log 404 connection
                    log("404", &log_file, file_path.as_str());

                    // TODO: add 404 error handling?
                    let status_line = "HTTP/1.1 404 NOT FOUND";
                    let four_path = format!("{}/{}", dir, not_found);
                    let html = fs::read_to_string(four_path).unwrap();
                    let response = format!("{status_line}\r\n\r\n{html}");

                    // send 404 page to recipient
                    stream.write_all(response.as_bytes()).unwrap();
                }
            };
        }
    };
}

// function to get values from config.txt
// TODO: find a way to make this private?
pub fn parse_config(desire: &str) -> String
{
    // read config file to string
    // TODO: find a way to make the file name variable?
    // problem is, that would have to be stored in config.txt itself
    // will stay hardcoded for now
    let file = fs::read_to_string("config.txt");

    // match Result above
    // TODO: learn better error handling
    // (don't want to have to open config.txt twice)
    // this applies elsewhere in codebase too
    match file
    {
        Ok(_) =>
        {
            // if file exists, open it again
            for line in fs::read_to_string("config.txt").unwrap().lines()
            {
                // search through all lines
                if line.find(desire).is_some()
                {
                    // once desired value is found, split string by "="
                    // then return nth(1) (since .split() returns an iterator)
                    let return_value = line.split("=").nth(1).unwrap().to_string();
                    return return_value
                }
            }
        },

        // if file doesn't exist
        Err(_) =>
        {
            // create it
            // TODO: while creating it, add some default values?
            // that way server can get up and running faster
            fs::File::create("config.txt").unwrap();
        }
    }
    
    // if for some reason the input is invalid, immediately panic
    // (which should never happen since they're hardcoded in, no user error possible)
    // TODO: find a better way to deal with this maybe?
    // probably not needed though, since this will only happen on dev error
    panic!("invalid request to config, panicking");
}
