use std::fs;
use std::io;
use std::path::Path;
use sysinfo::{System, SystemExt, CpuExt};


fn list_files_in_current_directory() -> io::Result<()> {
    let current_dir = std::env::current_dir()?;
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            println!("{}", path.display());
        }
    }
    Ok(())
}


fn delete_file(filename: &str) -> io::Result<()> {
    fs::remove_file(filename)?;
    println!("Deleted file: {}", filename);
    Ok(())
}


fn change_directory(dir: &str) -> io::Result<()> {
    std::env::set_current_dir(dir)?;
    println!("Changed directory to: {}", dir);
    Ok(())
}


fn main() {
   
    loop
    {
        let mut cmd = String::new();
        std::io::stdin().read_line(&mut cmd).unwrap();
        let cmd = cmd.trim();

        if cmd == "./ls"
        {
            if let Err(e) = list_files_in_current_directory() 
            {
                eprintln!("Error listing files: {}", e);
            }
        }
        if cmd == "./rm"
        {
            let mut file_name = String::new();
            println!("Enter in File to be Deleted: ");
            std::io::stdin().read_line(&mut file_name).unwrap();
            let file_name = file_name.trim();
            if let Err(e) = delete_file(file_name) 
            {
                eprintln!("Error deleting file: {}", e);
            }
        }

        if cmd == "./cd"
        {
            let mut directory_to_change = String::new();
            println!("Enter in Directory to Change To: ");
            std::io::stdin().read_line(&mut directory_to_change);
            let directory_to_change = directory_to_change.trim();
            if let Err(e) = change_directory(directory_to_change) {
                eprintln!("Error changing directory: {}", e);
            }
        }
        if cmd == "./stats"
        {
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
            

        
        if cmd == "./exit"
        {
            break
        }
    }
    std::process::exit(0);
}
