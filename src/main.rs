use std::process::{Command, exit};
use names::Generator;

fn ensure_git_installed() {
    if Command::new("git")
        .arg("--version")
        .output()
        .is_err()
    {
        eprintln!("Error: Git is not installed or not found in PATH.");
        exit(1);
    }
}

fn git_automate() {
    // git add -A
    let add_command = Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("Failed to execute git add command");

    if !add_command.status.success() {
        eprintln!(
            "Error: Failed to add files to the git repo.\n{}",
            String::from_utf8_lossy(&add_command.stderr)
        );
        exit(1);
    }

    // Check if there is anything staged to commit
    let nothing_to_commit = Command::new("git")
        .arg("diff")
        .arg("--cached")
        .arg("--quiet")
        .output()
        .expect("Failed to check git status");

    if nothing_to_commit.status.success() {
        println!("Nothing to commit, working tree clean.");
        return;
    }

    let commit_message = format!("auto: {}", name_generator());

    // git commit -m "message"
    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&commit_message)
        .output()
        .expect("Failed to execute git commit command");

    if !commit_command.status.success() {
        eprintln!(
            "Error: Failed to commit changes.\n{}",
            String::from_utf8_lossy(&commit_command.stderr)
        );
        exit(1);
    }

    // git push origin master
    let push_command = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("master")
        .output()
        .expect("Failed to execute git push command");

    if !push_command.status.success() {
        eprintln!(
            "Error: Failed to push changes to remote.\n{}",
            String::from_utf8_lossy(&push_command.stderr)
        );
        exit(1);
    }

    println!(
        "Successfully added, committed (\"{}\"), and pushed all changes!",
        commit_message
    );
}

fn name_generator() -> String {
    let mut generator = Generator::default();
    generator.next().unwrap()
}

fn main() {
    ensure_git_installed();
    git_automate();
}