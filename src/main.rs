use std::process::Command;

fn main() {
    println!("Yeah!.. Let's crack the shell!..");
    loop {
        let mut cmd = String::new();
        if let Ok(size) = std::io::stdin().read_line(&mut cmd) {
            let cmd = cmd.trim_matches(char::is_whitespace);
            if ! cmd.contains(char::is_whitespace) {
                let mut program= Command::new(cmd);
                match program.spawn() {
                    Ok(child) => {
                        #[cfg(debug_assertions)]
                        println!("Launched: {:?}!..", child);
                    },
                    Err(e) => panic!("Couldn't find {} in PATH.\n{}", &cmd[..], e)
                }
            }
        }
    }
}
