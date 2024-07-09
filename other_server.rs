use std::io::{self, Write};
use std::io::{Read};
use std::net::TcpStream;
use std::net::TcpListener;
use serde_json::Value;
use serde_json::json;
use std::thread;
use sysinfo::System;
use std::fs::File;
use sysinfo::SystemExt;
use sysinfo::CpuExt;

fn change_directory(dir: &str) -> io::Result<()> {
    std::env::set_current_dir(dir)?;
    println!("Changed directory to: {}", dir);
    Ok(())
}



fn list_files_in_current_directory() -> io::Result<()> {
    let current_dir = std::env::current_dir()?;
    for entry in std::fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            println!("{}", path.display());
        }
    }
    Ok(())
}

fn list_directories_in_current_directory() -> io::Result<()> {
    let current_dir = std::env::current_dir()?;
    for entry in std::fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            println!("{}", path.display());
        }
    }
    Ok(())
}



fn show_stats() {
    let mut system = System::new_all();
    system.refresh_all();
    let total_memory = system.total_memory();
    let used_memory = system.used_memory();
    let memory_usage_percentage = (used_memory as f64 / total_memory as f64) * 100.0;
    system.refresh_cpu();
    let global_cpu_usage = system.global_cpu_info().cpu_usage();
    println!("Memory usage: {:.2}%", memory_usage_percentage);
    println!("Global CPU usage: {:.2}%", global_cpu_usage);
}


fn create_file(filename: &str) -> io::Result<()> {
    File::create(filename)?;
    println!("Created file: {}", filename);
    Ok(())
}



fn fetch_remote_stats(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(host)?;

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer)?;

    let stats: Value = serde_json::from_slice(&buffer)?;
    println!("Remote stats: {}", stats);
    Ok(())
}


fn handle_connection_requests(mut stream: std::net::TcpStream) -> Result<(), Box<dyn std::error::Error>>
{
    let mut buffer = Vec::new();
    let mut database = vec!["Joey", "Craig", "bob"];
    stream.read_to_end(&mut buffer)?;


    //let per_addr = stream.peer_addr()?;
    let request: Value = serde_json::from_slice(&buffer)?;
    println!("Request: {}", request);

    let request = request.to_string();
    let request = request.trim();
    let get_req: &str = "GET";

    let post_req: &str = "POST";
    match request 
    {
        get_req =>
        {
            let per_addr = stream.peer_addr()?;
            let revo = per_addr.ip();
            let beligerant = json!({
                "Name_1": database[0],
                "Name_2": database[1],
            });

            let returnable = beligerant.to_string();

            let mut stream2 = TcpStream::connect(revo.to_string());

            stream2?.write_all(returnable.as_bytes()).unwrap()
        }

        post_req =>
        {

        }

        _ => {println!("Nay");}
    }

    Ok(())
}

fn handle_post_get_req(mut stream: std::net::TcpStream) -> Result<(), Box<dyn std::error::Error>> 
{
    let mut buffer = Vec::new();

    stream.read_to_end(&mut buffer)?;

    let returnable: Value = serde_json::from_slice(&buffer)?;
    println!("Returned info: {}", returnable);

    Ok(())
}



fn handle_connection_stats(mut stream: std::net::TcpStream) {
    let mut system = System::new_all();
    system.refresh_all();

    let total_memory = system.total_memory();
    let used_memory = system.used_memory();
    let memory_usage_percentage = (used_memory as f64 / total_memory as f64) * 100.0;

    system.refresh_cpu();
    let global_cpu_usage = system.global_cpu_info().cpu_usage();

    let stats = json!({
        "memory_usage": memory_usage_percentage,
        "cpu_usage": global_cpu_usage
    });

    let response = stats.to_string();
    stream.write_all(response.as_bytes()).unwrap();
}


fn main() {
    

    println!("Unreal!");

    let mut database = vec!["Joey", "Craig", "bob"];

    loop
    {
        let mut cmd = String::new();
        std::io::stdin().read_line(&mut cmd).unwrap();
        let cmd = cmd.trim();

        match cmd 
        {
            "ls" => 
            {
                if let Err(e) = list_files_in_current_directory() {
                    eprintln!("Error listing files: {}", e);
                }
                if let Err(e) = list_directories_in_current_directory() {
                    eprintln!("Error listing directories: {}", e);
                }
            }
            "./start server stats" =>
            {
                let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
                println!("Server listening on port 7878");

                for stream in listener.incoming()
                {
                    let stream = stream.unwrap();
                    thread::spawn(|| {
                        handle_connection_stats(stream);
                    });
                }
            }


            "./start server request" =>
            {
                let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
                println!("Server searching for requests on port 7878");
                
                for stream in listener.incoming()
                {
                    let stream = stream.unwrap();
                    
                    thread::spawn(|| {
                        handle_connection_requests(stream);
                    });
                }
            }
            "cd" =>
            {
                let mut directory_to_change = String::new();
                println!("Enter directory to change to: ");
                std::io::stdin().read_line(&mut directory_to_change).unwrap();
                let directory_to_change = directory_to_change.trim();
                if let Err(e) = change_directory(directory_to_change) {
                    eprintln!("Error changing directory: {}", e);
                }
            }

            "./get server stats" =>

            {

                let mut host = String::new();
                println!("Enter remote host (e.g., 192.168.1.100:7878): ");
                std::io::stdin().read_line(&mut host).unwrap();
                let host = host.trim();
                if let Err(e) = fetch_remote_stats(host) {
                    eprintln!("Error showing remote stats: {}", e);
                }
            }



            "./request" =>
            {
                let mut req = String::new();
                println!("Enter in the type of request: ");
                std::io::stdin().read_line(&mut req).unwrap();
                let req = req.trim();

                let mut ip = String::new();
                println!("Enter in the ip to send the request to: " );
                std::io::stdin().read_line(&mut ip).unwrap();
                let ip = ip.trim();

                let mut stream = TcpStream::connect(ip);

                let to_send = json!({
                    "type_of_request": req,

                });

                let response = to_send.to_string();

                stream.expect("REASON").write_all(response.as_bytes()).unwrap();

                match req
                {
                    "GET" =>
                    {
                        let mut stream2 = TcpStream::connect(ip);
                        handle_post_get_req(stream2.expect("REASON"));
                    }
                    _ => {println!("L");}
                }
            }


            "./stats" =>
            {
                show_stats();
            }



            "cr" => {
                let mut file_name = String::new();
                println!("Enter file to be created: ");
                std::io::stdin().read_line(&mut file_name).unwrap();
                let file_name = file_name.trim();
                if let Err(e) = create_file(file_name) {
                    eprintln!("Error creating file: {}", e);
                }
            }





            "./pnpm" =>
            {
                println!("not done yet");

            }



            "exit" =>
            {
                break;
            }

            "./exec" =>
            {
                let mut cmd = String::new();
                std::io::stdin().read_line(&mut cmd).unwrap();
                let cmd = cmd.trim();
                match cmd
                {
                    "ls" =>
                    {
                        if let Err(e) = list_files_in_current_directory() {
                            eprintln!("Error listing files: {}", e);
                        }
                    }
                    _ => 
                    { println!("unknown"); }
                }


            }
            _ => 
            {
                println!("Unknown");
            }
        }
    }    

    std::process::exit(0);


}
