mod server;
mod console;

// handle_connection is used decide what to do for incoming connections to server,
// then handle them
// parse_config looks at the config file to get a sort of env var
use server::{
    handle_connection,

    // TODO: find a way to use parse_config without making it public?
    parse_config
};

// server console function
use console::console;

// listen for incoming tcp connections
use std::net::TcpListener;

// threadpool crate
use threadpool::ThreadPool;

fn main()
{
    // get ip address from config file, format it with port num
    let ip = format!("{}:{}", parse_config("IP_ADDRESS"), parse_config("PORT_NUM"));
    
    // bind tcp listener to above ip
    let listener = TcpListener::bind(ip).unwrap();

    // create new threadpool with number of threads from config
    // TODO: error handling in case of incorrect thread nums?
    let pool = ThreadPool::new(parse_config("NUM_THREADS").parse().unwrap());

    // start new thread for server console, with log file path and server name
    pool.execute(||
        {
            console(parse_config("LOG_FILE").as_str(), parse_config("SERVER_NAME").as_str(), 
            parse_config("BACKUP_LOG").as_str(), parse_config("HISTORY_FILE").as_str());
        }
    );

    // for every incoming connection, start new thread and handle it
    for stream in listener.incoming()
    {
        // TODO: error handling instead of .unwrap()?
        let stream = stream.unwrap();
        
        pool.execute(||
            {
                handle_connection(stream);
            }
        );
    }
}
