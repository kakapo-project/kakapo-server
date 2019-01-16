
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

pub struct Scripting;

//TODO: this takes too long the first time
fn build_image(tag: &str) {
    let mut path_dir = PathBuf::from("/home/atta/.kakapo/scripts");
    path_dir.push(tag);
    let path = path_dir.to_str().unwrap();

    //TODO: should probably use the /var/run/docker.sock API
    let result = Command::new("docker")
        .arg("build")
        .arg(path)
        .arg(&format!("--tag={}", tag))
        .output()
        .expect("docker failed to start");

    println!("result: {:?}", result);

}

fn build_directory(name: &str, script_text: &str) {

    let docker_script = format!(r#"
FROM python:3.7
COPY . /app

CMD python /app/{name}.py
"#, name=name);

    let mut path_dir = PathBuf::from("/home/atta/.kakapo/scripts");
    path_dir.push(name);
    fs::create_dir_all(path_dir.to_owned()).expect("could not create file");

    let mut docker_file = path_dir.to_owned();
    docker_file.push("Dockerfile");
    fs::write(docker_file, docker_script).expect("could not create docker file");

    let mut script_file = path_dir.to_owned();
    script_file.push(&format!("{}.py", name));
    fs::write(script_file, script_text).expect("could not create script file");
}

fn delete_image(tag: &str) {

    let result = Command::new("docker")
        .arg("image")
        .arg("rm")
        .arg("-f")
        .arg(tag)
        .output()
        .expect("docker failed to start");

    println!("result: {:?}", result);

}

fn delete_directory(name: &str) {

    let mut path_dir = PathBuf::from("/home/atta/.kakapo/scripts");
    path_dir.push(name);
    fs::remove_dir_all(path_dir); //try deleting
}

fn run_container(tag: &str) {
    //TODO: should probably use the /var/run/docker.sock API
    let result = Command::new("docker")
        .arg("run")
        //.arg("-v")
        //.arg("~/tmp:/var/commfile.txt")
        .arg(tag)
        .output()
        .expect("docker failed to start");

    println!("result: {:?}", result);
}

impl ScriptFunctions for Scripting {

    fn build() {
        debug!("building the image");
        //delete image if exists
        delete_image("the_room");

        //delete directory
        delete_directory("the_room");

        //make dockerfile
        build_directory("the_room", "print('oh hi doggy')");

        //docker build >> sudo docker build . --tag={{name}}
        build_image("the_room");
        unimplemented!();

    }

    fn delete() {
        //delete image if exists
        delete_image("the_room");

        //delete directory
        delete_directory("the_room");
        unimplemented!();
    }

    fn run() {
        //check if image exists
        //run container >> sudo docker run -v {{tmpfile}}:/var/commfile.txt {{name}}

        run_container("the_room");
        unimplemented!();

    }
}
