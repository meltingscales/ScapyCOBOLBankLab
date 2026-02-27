use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use std::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[[ SCAPY COBOL BANK SERVER ]]");
    println!("Listening on 127.0.0.1:9999");
    println!("================================");

    let listener = TcpListener::bind("127.0.0.1:9999").await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("[+] Connection from: {}", addr);

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut buf_reader = BufReader::new(reader);
            let mut line = String::new();

            while buf_reader.read_line(&mut line).await.unwrap() > 0 {
                let command = line.trim();
                println!("[RECV] {}", command);

                let response = process_command(command);
                writer.write_all(response.as_bytes()).await.unwrap();
                writer.write_all(b"\n").await.unwrap();

                line.clear();
            }
            println!("[-] Client disconnected: {}", addr);
        });
    }
}

fn process_command(input: &str) -> String {
    // Parse: TYPE|SUBTYPE|TO|FROM|NOTE1|NOTE2
    let parts: Vec<&str> = input.split('|').collect();

    if parts.len() < 6 {
        return format!("ERROR: Invalid command format");
    }

    let cmd_type = parts[0];
    let subtype = parts[1];
    let _to = parts[2];
    let _from = parts[3];
    let note1 = parts[4];
    let note2 = parts[5];

    match (cmd_type, subtype) {
        ("BANKDATA", "XFER") => {
            // Safe bank data transfer - just log it
            println!("[BANKDATA] Transfer: {} -> {} | Amount: {}", parts[2], parts[3], note1);
            format!("OK: BANKDATA XFER logged")
        }
        ("BANKCMD", "CPY") => {
            // VULNERABLE: Command injection in copy operation
            println!("[BANKCMD] CPY: {} -> {}", note1, note2);

            // VULNERABLE CODE - Using shell command with user input
            let output = if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .args(["/C", "copy", note1, note2])
                    .output()
            } else {
                Command::new("sh")
                    .arg("-c")
                    .arg(format!("cp {} {}", note1, note2))
                    .output()
            };

            match output {
                Ok(o) => {
                    if o.status.success() {
                        format!("OK: File copied")
                    } else {
                        format!("ERROR: Copy failed - {}", String::from_utf8_lossy(&o.stderr))
                    }
                }
                Err(e) => format!("ERROR: {}", e)
            }
        }
        ("BANKCMD", "RUN") => {
            // VULNERABLE: Direct command execution
            println!("[BANKCMD] RUN: {}", note1);

            // VULNERABLE CODE - Executing user input directly
            let output = if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .args(["/C", note1])
                    .output()
            } else {
                Command::new("sh")
                    .arg("-c")
                    .arg(note1)
                    .output()
            };

            match output {
                Ok(o) => {
                    let stdout = String::from_utf8_lossy(&o.stdout);
                    let stderr = String::from_utf8_lossy(&o.stderr);
                    format!("OK: CMD executed\nSTDOUT: {}\nSTDERR: {}", stdout, stderr)
                }
                Err(e) => format!("ERROR: {}", e)
            }
        }
        _ => {
            format!("ERROR: Unknown command: {}|{}", cmd_type, subtype)
        }
    }
}
