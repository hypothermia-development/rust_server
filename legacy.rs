use std::io::{self, Write, Read};
use std::net::TcpStream;
use std::fs::File;
use sysinfo::{System, SystemExt, CpuExt};
use serde_json::Value;

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

fn delete_file(filename: &str) -> io::Result<()> {
    std::fs::remove_file(filename)?;
    println!("Deleted file: {}", filename);
    Ok(())
}

fn delete_dir(dir_name: &str) -> io::Result<()> {
    std::fs::remove_dir(dir_name)?;
    println!("Deleted dir: {}", dir_name);
    Ok(())
}

fn create_file(filename: &str) -> io::Result<()> {
    File::create(filename)?;
    println!("Created file: {}", filename);
    Ok(())
}

fn change_directory(dir: &str) -> io::Result<()> {
    std::env::set_current_dir(dir)?;
    println!("Changed directory to: {}", dir);
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

fn fetch_remote_stats(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(host)?;

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer)?;

    let stats: Value = serde_json::from_slice(&buffer)?;
    println!("Remote stats: {}", stats);
    Ok(())
}

fn string_to_i32(input: &str) -> Result<i32, std::num::ParseIntError> {
    let parsed_int = input.trim().parse::<i32>()?;
    Ok(parsed_int)
}


struct server
{
    pub id: u32,
    pub name: String,
    pub desc: String,
    pub ip: String,

}

impl server
{
    fn new(id: u32, name: String, desc: String, ip: String) -> Self
    {
        Self {id, name, desc, ip}
    }

    fn display_info(&self)
    {
        println!("Server Name {}", self.name);
        println!("Server Description: {}", self.desc);
        println!("Server Ip: {}", self.ip);
    }
    fn get_ip(&self) -> String 
    {
        return self.ip.clone();
    }
  //  fn fetch_remote_stats(host: &str) -> Result<(), Box<dyn std::error::Error>> {
  //      let mut stream = TcpStream::connect(host)?;
  //      let mut buffer = Vec::new();
  //      stream.read_to_end(&mut buffer)?;
//
  //      let stats: Value = serde_json::from_slice(&buffer)?;
  //      println!("Remote stats: {}", stats);
  //      Ok(())
  //  }
}

fn main() {
    loop {
        let mut server_id: u32 = 1;
        let mut servers: Vec<server> = Vec::new();
        let mut cmd = String::new();
        std::io::stdin().read_line(&mut cmd).unwrap();
        let cmd = cmd.trim();

        match cmd {
            "./ls" => {
                if let Err(e) = list_files_in_current_directory() {
                    eprintln!("Error listing files: {}", e);
                }
                if let Err(e) = list_directories_in_current_directory() {
                    eprintln!("Error listing directories: {}", e);
                }
            }
            "./rm" => {
                let mut file_name = String::new();
                println!("Enter file to be deleted: ");
                std::io::stdin().read_line(&mut file_name).unwrap();
                let file_name = file_name.trim();
                if let Err(e) = delete_file(file_name) {
                    eprintln!("Error deleting file: {}", e);
                }
            }
            "./rm dir" => {
                let mut dir_name = String::new();
                println!("Enter directory to be deleted: ");
                std::io::stdin().read_line(&mut dir_name).unwrap();
                let dir_name = dir_name.trim();
                if let Err(e) = delete_dir(dir_name) {
                    eprintln!("Error deleting directory: {}", e);
                }
            }
            "./cr" => {
                let mut file_name = String::new();
                println!("Enter file to be created: ");
                std::io::stdin().read_line(&mut file_name).unwrap();
                let file_name = file_name.trim();
                if let Err(e) = create_file(file_name) {
                    eprintln!("Error creating file: {}", e);
                }
            }
            "./cd" => {
                let mut directory_to_change = String::new();
                println!("Enter directory to change to: ");
                std::io::stdin().read_line(&mut directory_to_change).unwrap();
                let directory_to_change = directory_to_change.trim();
                if let Err(e) = change_directory(directory_to_change) {
                    eprintln!("Error changing directory: {}", e);
                }
            }
            "./stats" => {
                show_stats();
            }
            "./remote_stats" => {
                let mut host = String::new();
                println!("Enter remote host (e.g., 192.168.1.100:7878): ");
                std::io::stdin().read_line(&mut host).unwrap();
                let host = host.trim();

                if let Err(e) = fetch_remote_stats(host) {
                    eprintln!("Error showing remote stats: {}", e);
                }
            }
            "./add server" =>{
                let mut temp_name = String::new();
                let mut temp_desc = String::new();
                let mut temp_ip = String::new();

                println!("Enter in server name: ");
                std::io::stdin().read_line(&mut temp_name).unwrap();
                let temp_name = temp_name.trim();

                println!("Enter in server description: ");
                std::io::stdin().read_line(&mut temp_desc).unwrap();
                let temp_desc = temp_name.trim();

                println!("Enter in server ip: ");
                std::io::stdin().read_line(&mut temp_ip).unwrap();
                let temp_ip = temp_ip.trim();

                let server = server::new(server_id, temp_name.to_string(), temp_desc.to_string(), temp_ip.to_string());
                servers.push(server);

                server_id += 1;

                for server in &servers
                {
                    server.display_info();
                    let a = server.get_ip();
                    println!("{}", a);
                }
            }
            
          //  "./server remote_stats" => {

           //     let mut idea: u32 = 0;
//
            //    println!("Enter in a server id: ");
              //  std::io::stdin().read_line(&mut idea).unwrap();
             //   let idea = idea.trim();

         //   }


            "./exit" => {
                break;
            }
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
    std::process::exit(0);
}

