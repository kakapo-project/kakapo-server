
use tokio::prelude::{Future, Stream};
use std::path::PathBuf;
use std::fs;
use std::env;
use std::process::Command;

pub trait ScriptFunctions {
    fn build();
    fn delete();
    fn run();
}

pub enum ScriptError {
    IOError(String),
    ExecuteError,
    RuntimeError(String),
}

pub struct Scripting;

fn u8vec_to_string(vec: Vec<u8>) -> String {
    vec.iter().map(|&c| c as char).collect::<String>()
}

//TODO: this takes too long the first time
fn build_image(script_home: &str, tag: &str) -> Result<(), ScriptError> {
    let mut path_dir = PathBuf::from(script_home);
    path_dir.push(tag);
    let path = path_dir.to_str().unwrap();

    //TODO: should probably use the /var/run/docker.sock API
    let result = Command::new("docker")
        .arg("build")
        .arg(path)
        .arg(&format!("--tag={}", tag))
        .output();

    result
        .or_else(|err| Err(ScriptError::ExecuteError))
        .and_then(|res| {
            info!("building image: ran docker command: {:?}", u8vec_to_string(res.stdout));
            if res.status.success() {
                Ok(())
            } else {
                let error_msg = u8vec_to_string(res.stderr);
                Err(ScriptError::RuntimeError(error_msg))
            }
        })

}

fn build_directory(script_home: &str, name: &str, script_text: &str) -> Result<(), ScriptError> {

    let docker_script = format!(r#"
FROM python:3.7
COPY . /app

CMD python /app/{name}.py
"#, name=name);

    let mut path_dir = PathBuf::from(script_home);
    path_dir.push(name);
    fs::create_dir_all(path_dir.to_owned())
        .or_else(|err| Err(
            ScriptError::IOError(format!("not able to create directory {:?}", &path_dir))))?;

    let mut docker_file = path_dir.to_owned();
    docker_file.push("Dockerfile");
    fs::write(docker_file, docker_script)
        .or_else(|err| Err(
            ScriptError::IOError(format!("not able to create file {:?}/Dockerfile", &path_dir))))?;

    let mut script_file = path_dir.to_owned();
    script_file.push(&format!("{}.py", name));
    fs::write(script_file, script_text)
        .or_else(|err| Err(
            ScriptError::IOError(format!("not able to create file {:?}/{}.py", &path_dir, &name))))?;

    Ok(())
}

fn delete_image(tag: &str) -> Result<(), ScriptError> {

    let result = Command::new("docker")
        .arg("image")
        .arg("rm")
        .arg("-f")
        .arg(tag)
        .output();

    result
        .or_else(|err| Err(ScriptError::ExecuteError))
        .and_then(|res| {
            info!("deleting image: ran docker command: {:?}", u8vec_to_string(res.stdout));
            if res.status.success() {
                Ok(())
            } else {
                let error_msg = u8vec_to_string(res.stderr);
                Err(ScriptError::RuntimeError(error_msg))
            }
        })

}

fn delete_directory(script_home: &str, name: &str) -> Result<(), ScriptError> {

    let mut path_dir = PathBuf::from(script_home);
    path_dir.push(name);
    fs::remove_dir_all(path_dir.to_owned())
        .or_else(|err| Err(
            ScriptError::IOError(format!("not able to remove directory {:?}", &path_dir))))?;

    Ok(())

}

fn run_container(tag: &str) -> Result<(), ScriptError> {
    //TODO: should probably use the /var/run/docker.sock API
    let result = Command::new("docker")
        .arg("run")
        //.arg("-v")
        //.arg("~/tmp:/var/commfile.txt")
        .arg(tag)
        .output();

    result
        .or_else(|err| Err(ScriptError::ExecuteError))
        .and_then(|res| {
            info!("ruynning container: ran docker command: {:?}", u8vec_to_string(res.stdout));
            if res.status.success() {
                Ok(())
            } else {
                let error_msg = u8vec_to_string(res.stderr);
                Err(ScriptError::RuntimeError(error_msg))
            }
        })
}

impl ScriptFunctions for Scripting {

    fn build() {
        let script_home = "/home/atta/.kakapo/scripts";
        debug!("building the image");
        //delete image if exists
        delete_image("the_room");

        //delete directory
        delete_directory(script_home, "the_room");

        //make dockerfile
        build_directory(script_home, "the_room", "print('oh hi doggy')");

        //docker build >> sudo docker build . --tag={{name}}
        build_image(script_home, "the_room");
        unimplemented!();

    }

    fn delete() {
        let script_home = "/home/atta/.kakapo/scripts";

        //delete image if exists
        delete_image("the_room");

        //delete directory
        delete_directory(script_home,"the_room");
        unimplemented!();
    }

    fn run() {
        let script_home = "/home/atta/.kakapo/scripts";
        //check if image exists
        //run container >> sudo docker run -v {{tmpfile}}:/var/commfile.txt {{name}}

        run_container("the_room");
        unimplemented!();

    }
}
