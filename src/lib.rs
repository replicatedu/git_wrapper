use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

//use hubcaps::repositories::{RepoOptions};

//returns a command setup ready to run the tests
pub fn command_wrapper(test_command: &str, command_directory: &str) -> Command {
    let mut command = if cfg!(target_os = "windows") {
        {
            let mut c = Command::new("cmd");
            c.args(&["/C", test_command]);
            c
        }
    } else {
        {
            let mut c = Command::new("sh");
            c.arg("-c");
            c.arg(test_command);
            c
        }
    };
    command.current_dir(command_directory);
    command
}

//rsa key generation
//ssh-keygen -f /etc/ssh/ssh_host_rsa_key -N '' -t rsa
pub fn gen_rsa_keys(path: &str) {
    let command = "rm -rf ./deploy_key* && \
                   ssh-keygen -f ./deploy_key -N '' -t rsa && \
                   echo \"paste the following into deploy keys\" && \
                   cat deploy_key.pub &&
                   ssh-add ./deploy_key";
    let mut c = command_wrapper(&command, ".");
    let c_out = c.output().expect("gen_rsa_keys failed");
    println!(
        "STD_OUT\n{}\nSTDERR\n{}",
        String::from_utf8_lossy(&c_out.stdout),
        String::from_utf8_lossy(&c_out.stderr)
    );
    let command = "rm -rf ./deploy_key* && \
                   ssh-keygen -f ./deploy_key -N '' -t rsa && \
                   echo \"paste the following into deploy keys\" && \
                   cat deploy_key.pub &&
                   ssh-add ./deploy_key";
    let mut c = command_wrapper(&command, path);
    let c_out = c.output().expect("gen_rsa_keys failed");
    println!(
        "STD_OUT\n{}\nSTDERR\n{}",
        String::from_utf8_lossy(&c_out.stdout),
        String::from_utf8_lossy(&c_out.stderr)
    );
}

pub fn pull_class_database(repo_address: &str, repo_path: &str) {
    let owned_string: String = "git clone ".to_owned();
    let command = owned_string + repo_address;
    println!("running: {}", command);
    let mut c = command_wrapper(&command, repo_path);
    let c_out = c.output().expect("pull_class_database failed");
    println!(
        "STD_OUT\n{}\nSTDERR\n{}",
        String::from_utf8_lossy(&c_out.stdout),
        String::from_utf8_lossy(&c_out.stderr)
    );
}

pub fn turn_off_host_checks() {
    let mut command: String = "mkdir -p ~/.ssh && ".to_owned();
    command += "echo \"Host *\" > ~/.ssh/config && ";
    command += "echo \" StrictHostKeyChecking no\" >> ~/.ssh/config";
    println!("running: {}", command);
    let mut c = command_wrapper(&command, "/");
    let c_out = c.output().expect("turn_off_host_checks failed");
    println!(
        "STD_OUT\n{}\nSTDERR\n{}",
        String::from_utf8_lossy(&c_out.stdout),
        String::from_utf8_lossy(&c_out.stderr)
    );
}

pub fn turn_on_host_checks() {
    let mut command: String = "rm ~/.ssh/config && ".to_owned();
    command += "echo \"Host *\" > ~/.ssh/config && ";
    command += "echo \" StrictHostKeyChecking no\" >> ~/.ssh/config";
    println!("running: {}", command);
    let mut c = command_wrapper(&command, "/");
    let c_out = c.output().expect("turn_on_host_checks failed");
    println!(
        "STD_OUT\n{}\nSTDERR\n{}",
        String::from_utf8_lossy(&c_out.stdout),
        String::from_utf8_lossy(&c_out.stderr)
    );
}

pub fn add_file(filename: &str, repo_path: &str) {
    let mut command: String = "".to_owned();
    command += "git pull ";
    command += " && git add ";
    command += &filename;
    command += " && git commit -a -m \"added a new file\" ";
    command += "&& git push origin master";
    println!("running: {}", command);
    let mut c = command_wrapper(&command, repo_path);
    let c_out = c.output().expect("add_file failed");
    println!(
        "STD_OUT\n{}\nSTDERR\n{}",
        String::from_utf8_lossy(&c_out.stdout),
        String::from_utf8_lossy(&c_out.stderr)
    );
}

pub fn add_files(repo_path: &str) {
    let mut command: String = "touch ".to_owned();
    command += "git pull ";
    command += " && git add .";
    command += " && git commit -a -m \"added a new file\" ";
    command += "&& git push origin master";
    println!("running: {}", command);
    let mut c = command_wrapper(&command, repo_path);
    let c_out = c.output().expect("add_file failed");
    println!(
        "STD_OUT\n{}\nSTDERR\n{}",
        String::from_utf8_lossy(&c_out.stdout),
        String::from_utf8_lossy(&c_out.stderr)
    );
}

pub fn pass_add_file(
    filename: &str,
    repo_path: &str,
    username: &str,
    password: &str,
    repo_name: &str,
) {
    let mut command: String = "".to_owned();
    command += "git pull ";
    command += " && git add ";
    command += &filename;
    command += " && git commit -a -m \"added a new file\" ";
    command.push_str(&format!(
        "&& git push --mirror https://{}:{}@github.com/{}/{}.git",
        username, password, username, repo_name
    ));
    println!("running: {}", command);
    let mut c = command_wrapper(&command, repo_path);
    let c_out = c.output().expect("add_file failed");
    println!(
        "STD_OUT\n{}\nSTDERR\n{}",
        String::from_utf8_lossy(&c_out.stdout),
        String::from_utf8_lossy(&c_out.stderr)
    );
}

pub fn pass_add_files(repo_path: &str, username: &str, password: &str, repo_name: &str) {
    let mut command: String = "touch ".to_owned();
    command += "git pull ";
    command += " && git add .";
    command += " && git commit -a -m \"added a new file\" ";
    command.push_str(&format!(
        "&& git push --mirror https://{}:{}@github.com/{}/{}.git",
        username, password, username, repo_name
    ));
    println!("running: {}", command);
    let mut c = command_wrapper(&command, repo_path);
    let c_out = c.output().expect("add_file failed");
    println!(
        "STD_OUT\n{}\nSTDERR\n{}",
        String::from_utf8_lossy(&c_out.stdout),
        String::from_utf8_lossy(&c_out.stderr)
    );
}

pub fn prune_files(repo_path: &str) {
    let command: String = "find . -not -name .git -exec rm -vf {} \\;".to_owned();
    let mut c = command_wrapper(&command, repo_path);
    let c_out = c.output().expect("prune_files failed");
    println!(
        "STD_OUT\n{}\nSTDERR\n{}",
        String::from_utf8_lossy(&c_out.stdout),
        String::from_utf8_lossy(&c_out.stderr)
    );
}

pub fn time_since_epoch() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis().to_string()
}

pub fn pull_class_repo(repopath: &str, folder: &str) {
    let owned_string: String = "git clone ".to_owned();
    let command = owned_string + repopath;
    let mut c = command_wrapper(&command, folder);
    let c_out = c.output().expect("add_file failed");
    println!(
        "STD_OUT\n{}\nSTDERR\n{}",
        String::from_utf8_lossy(&c_out.stdout),
        String::from_utf8_lossy(&c_out.stderr)
    );
}

pub fn create_repo(username: &str, password: &str, repo_name: &str, path: &str) {
    //curl --url url -K- <<< "--user user:password"
    let mut command = String::new();
    command.push_str("curl --url https://api.github.com/user/repos ");
    command.push_str(&format!(
        "-d '{{\"name\":\"{}\",\"private\":true}}' ",
        repo_name
    ));
    command.push_str(&format!("--user \"{}:{}\"", username, password));
    println!("{}", command);

    let mut c = command_wrapper(&command, path);
    let c_out = c.output().expect("create_repo failed");
    println!(
        "STD_OUT\n{}\nSTDERR\n{}",
        String::from_utf8_lossy(&c_out.stdout),
        String::from_utf8_lossy(&c_out.stderr)
    );
}

//https://developer.github.com/v3/repos/keys/
//POST /repos/:owner/:repo/keys
pub fn add_deploy_key(username: &str, password: &str, repo_name: &str, path: &str, key: &str) {
    //curl --url url -K- <<< "--user user:password"
    let mut command = String::new();
    command.push_str(&format!(
        "curl --url https://api.github.com/repos/{}/{}/keys ",
        username, repo_name
    ));
    command.push_str(&format!(
        "-X POST -d '{{\"title\":\"instructor_key@key\",\"key\":\"{}\",\"read_only\":false}}' ",
        key.trim()
    ));
    command.push_str(&format!("--user \"{}:{}\"", username, password));
    println!("{}", command);

    let mut c = command_wrapper(&command, path);
    let c_out = c.output().expect("add_deploy_key failed");
    println!(
        "STD_OUT\n{}\nSTDERR\n{}",
        String::from_utf8_lossy(&c_out.stdout),
        String::from_utf8_lossy(&c_out.stderr)
    );
}

// git clone --bare https://github.com/exampleuser/public-repo.git
// cd public-repo.git
// git push --mirror https://github.com/yourname/private-repo.git
// cd ..
// rm -rf public-repo.git
pub fn clone_repo_to_private(
    username: &str,
    password: &str,
    repo_name: &str,
    path: &str,
    class_repo_address: &str,
) {
    let mut command = String::new();
    command.push_str(&format!("rm -rf {}.git && ", repo_name));
    command.push_str(&format!(
        "git clone --bare {} {} && ",
        class_repo_address, repo_name
    ));
    command.push_str(&format!("cd {} && ", repo_name));
    command.push_str(&format!(
        "git push --mirror https://{}:{}@github.com/{}/{}.git && ",
        username, password, username, repo_name
    ));
    command.push_str("cd .. && ");
    command.push_str(&format!("rm -rf {}", repo_name));
    println!("{}", command);

    let mut c = command_wrapper(&command, path);
    let c_out = c.output().expect("clone_repo_to_private failed");
    println!(
        "STD_OUT\n{}\nSTDERR\n{}",
        String::from_utf8_lossy(&c_out.stdout),
        String::from_utf8_lossy(&c_out.stderr)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::File;
    use std::io::prelude::*;
    #[test]
    fn test_create_repo() {
        let test_repo = "https://github.com/replicatedu/test_class";
        let repo_name = "test_class_hortinstein";
        let username = "hortinstein";
        let path = "/tmp/";
        let password = env::var("GITHUB_PASSWORD").expect("set the GITHUB_PASSWORD env");
        let key = "NO KEY";

        create_repo(username, &password, repo_name, path);
        gen_rsa_keys("/tmp");
        let mut file = File::open("/tmp/deploy_key.pub").expect("key not there");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("error reading key");
        add_deploy_key(username, &password, repo_name, path, &contents);
    }

}
