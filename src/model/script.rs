
use actix::prelude::*;
use cpython::{Python, PyDict, PyErr, PyResult, NoArgs};
use diesel;
use diesel::result::Error;
use diesel::{
    prelude::*,
    sql_types,
    insert_into,
    delete,
    update,
};
use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};
use diesel::sql_types::*;

use failure::Fail;
use serde_json;
use std::io::Write;
use std::io;
use std::collections::BTreeMap;
use std;
use std::fs::File;
use std::path::Path;
use std::ops::Deref;

use super::data;
use super::data::DataType;
use super::api;

use super::dbdata::*;
use super::schema::{entity, query, query_history};
use super::manage::retrieve::{get_one_script};
use super::database;
use super::connection;


/*
use cpython::{Python, PyDict, PyErr, PyResult, NoArgs};


fn main() {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let result: Result<(), PyErr> = hello(py)
        .or_else(|err| {
            err.print(py);
            Ok(())
        });

}

fn hello(py: Python) -> PyResult<()> {
    println!("running python");
    let result: String = py.eval("str(2 + 2)", None, None)?.extract(py)?;
    println!("output should equal 4 = {}", result);

    return Ok(());

    let sys = py.import("sys")?;
    let locals = PyDict::new(py);
    locals.set_item(py, "sys", sys)?;
    locals.set_item(py, "path", "/home/atta/ninchy/scripts")?;
    py.eval("sys.path.append(path)", None, Some(&locals))?;

    let runner = py.import("runner")?;
    locals.set_item(py, "runner", runner)?;
    py.eval("runner.run()", None, Some(&locals))?;

    Ok(())
}


fn with_docker(py: Python) -> PyResult<()> {

    let docker = py.import("docker")?;

    let client = docker.call(py, "from_env", NoArgs, None)?;

    let locals = PyDict::new(py);
    locals.set_item(py, "client", client)?;
    py.eval("print(client.containers.run('ubuntu:latest', 'echo hello world'))", None, Some(&locals))?;

    Ok(())
}
*/

fn setup_py(script_path: String, py: Python) -> PyResult<()> {

    let sys = py.import("sys")?;

    let locals = PyDict::new(py);
    locals.set_item(py, "sys", sys)?;
    locals.set_item(py, "path", script_path)?;

    py.eval("sys.path.append(path)", None, Some(&locals))?;

    return Ok(());
}

fn run_script_locally(py: Python, script_name: String) -> PyResult<String> {
    let module = py.import(&script_name)?;
    let json = py.import("json")?;
    let py_result = module.call(py, "handle", NoArgs, None)?;

    let locals = PyDict::new(py);
    locals.set_item(py, "json", json)?;
    locals.set_item(py, "result", py_result)?;

    let result: String = py.eval("json.dumps(result)", None, Some(&locals))?.extract(py)?;
    Ok(result)
}

fn handle_py_error() -> () {

}

pub fn run_script(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    py_runner: connection::py::PyRunner,
    script_name: String,
    params: data::ScriptParam,
) -> Result<api::RunScriptResult, api::Error> {

    //get table
    let script_item = conn.transaction::<data::Script, diesel::result::Error, _>(|| {

        let script_item = get_one_script(conn, script_name)?;
        println!("parsed_scripts: {:?}", script_item);

        Ok(script_item)
    })
    .or_else(|err| match err {
        diesel::result::Error::NotFound => Err(api::Error::ScriptNotFound),
        _ => Err(api::Error::DatabaseError(err)),
    })?;

    //dump table to file: TODO: figure out how to make this more thread safe
    let script_path = py_runner.get_script_path();
    let file_path = Path::new(&script_path).join(&script_item.name).with_extension("py");
    let mut file = File::create(file_path)
        .or_else(|err| Err(api::Error::UnknownError))?;
    file.write_all(script_item.text.as_bytes())
        .or_else(|err| Err(api::Error::UnknownError))?;

    let gil = Python::acquire_gil();
    let py = gil.python();

    setup_py(script_path, py).or_else(|mut err| {
        err.instance(py).extract(py)
            .or_else(|inner_err| {
                Err(api::Error::UnknownError)
            })
            .and_then(|message| {
                Err(api::Error::ScriptError(message))
            })
    })?;

    let json_result = run_script_locally(py, script_item.name).or_else(|mut err| {
        err.instance(py).extract(py)
            .or_else(|inner_err| {
                Err(api::Error::UnknownError)
            })
            .and_then(|message| {
                Err(api::Error::ScriptError(message))
            })
    })?;

    let result = serde_json::to_value(json_result).or_else(|err| Err(api::Error::UnknownError))?;

    Ok(api::RunScriptResult(result))
}

