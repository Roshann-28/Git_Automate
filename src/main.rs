use std::process::{Command, exit};
use names::Generator;

fn git_automate(){
    // git add -A
    let add_command = Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("Failed to execute git add Command");

    if !add_command.status.success(){
        eprintln!("Error: Failed to add files to the git repo .");
        exit(1);
    }
    // git commit -m "Your message"
    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(name_generator())
        .output()
        .expect("Failed to execute git commit command ");

    if !commit_command.status.success(){
        eprintln!("Error: Failed to commit changes .");
        exit(1);
    }

    // git push origin master
    // origin -> remote repo
    let push_command = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("master")
        .output()
        .expect("Failed to execute git push command");
    

    if !push_command.status.success(){
        eprintln!("Error: Failed to push changes to remote. ");
        exit(1);
    }

    println!("Succesfully added, commited, and pushed all changes! ");
}

fn name_generator() -> String{
    let mut generator = Generator::default();
    generator.next().unwrap()
}


fn main() {
    git_automate();
}
