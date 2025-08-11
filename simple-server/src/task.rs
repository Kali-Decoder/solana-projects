use csv::{Reader,Writer};
use serde::{Deserialize,Serialize};
use std::fs::File;
use std::io::Read;


#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Task {
    pub task_name:String,
    pub task_description:String,
    pub task_complete:bool
}



pub fn save_task(tasks:&Vec<Task>){
    let file = File::create("task.csv").unwrap();
    let mut writer = Writer::from_writer(file);
    for task in tasks {
        writer.serialize(task).unwrap();
    }
}

pub fn load_task() -> Vec<Task> {
    let mut tasks : Vec<Task> = vec![];
    let mut file = File::open("task.csv").unwrap_or_else(|_|
        File::create("task.csv").unwrap());

    let mut contents = String::new();

    match file.read_to_string(&mut contents){
        Ok(_)=>{
            let mut reader = Reader::from_reader(contents.as_bytes());
            for result in reader.deserialize(){
                let _tasks:Task = result.unwrap();
                print!("{:?}",&contents);
                tasks.push(_tasks);
            }
        }
        Err(_)=>(), 
    }
    tasks
}