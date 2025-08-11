use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::*;
use std::path::{Path, PathBuf};
use task::*;
mod task;

#[get("/tasks")]
fn get_tasks() -> Json<Vec<Task>> {
    let tasks = load_task();
    Json(tasks)
}

#[post("/tasks", format = "json", data = "<task>")]
fn create_task(task: Json<Task>) -> Status {
    let mut tasks = load_task();

    if let Some(_index) = tasks
        .iter()
        .position(|item| item.task_name == task.0.task_name)
    {
        return Status::Conflict;
    }

    tasks.push(task.0);
    save_task(&tasks);

    Status::Created
}

#[put("/task",format = "json",data="<task>")]
fn update_task(task: Json<Task>) -> Status{
    let mut tasks = load_task();
    if let Some(index) = tasks.iter().position(|item| item.task_name==task.0.task_name){
        tasks.remove(index);
        tasks.insert(index,task.0);
        save_task(&tasks);
        return Status::Ok;
    }else{
        return Status::NotFound;
    }
}

#[delete("/task",format="json",data="<task>")]
fn task_delete(task: Json<Task>) -> Status{
    let mut tasks = load_task();
    if let Some(index) = tasks.iter().position(|item| item.task_name==task.0.task_name){
        tasks.remove(index);
        save_task(&tasks);
        return Status::NoContent;
    }else{
        return Status::NotFound;
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_tasks, create_task,update_task,task_delete])
}
