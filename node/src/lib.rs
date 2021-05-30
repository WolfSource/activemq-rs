#[macro_use]
extern crate lazy_static;

use neon::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;

struct AMQNode {
    ptr: *mut std::os::raw::c_void,
}

impl Finalize for AMQNode {}

impl Drop for AMQNode {
    fn drop(&mut self) {
        println!("Dropping ActiveMQ instance");
    }
}
unsafe impl Send for AMQNode {}
unsafe impl Sync for AMQNode {}

lazy_static! {
    static ref MAPS: Mutex<HashMap<i32, AMQNode>> = Mutex::new(HashMap::new());
}

fn init(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    activemq_rs::amq::init();
    Ok(cx.boolean(true))
}

fn fini(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    activemq_rs::amq::fini();
    Ok(cx.boolean(true))
}

fn new_instance(mut cx: FunctionContext) -> JsResult<JsNumber> {
    use activemq_rs::amq::*;

    // get argument
    let conn_type_index = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let conn_type: amq_connection_type = match conn_type_index {
        0.0 => amq_connection_type::AMQ_CONSUMER,
        _ => amq_connection_type::AMQ_PRODUCER,
    };

    //create an amq instance, then box it
    let ins = activemq_rs::amq::new_instance(conn_type);
    let ins_boxed = Box::new(ins);
    let amq = AMQNode {
        ptr: Box::into_raw(ins_boxed) as *mut std::os::raw::c_void,
    };

    // lock the mutex and insert this activemq to it
    let mut map = MAPS.lock().unwrap();
    let index = map.len() + 1;
    map.insert(0, amq);

    Ok(cx.number(index as i32))
}

pub fn run(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    use activemq_rs::amq::*;

    // done
    let done: bool;
    // get unique index from argument
    let index = cx.argument::<JsNumber>(0)?.value(&mut cx) as i32;

    let map = MAPS.lock().unwrap();
    match map.get(&index) {
        Some(node) => {
            let amq_node = unsafe { Box::from_raw(node.ptr as *mut activemq_rs::AmqObj) };
            amq_node.run();
            println!("OK");
            done = true;
        }
        _ => {
            done = false;
        }
    }

    //amq_node.run();
    Ok(cx.boolean(done))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("init", init)?;
    cx.export_function("fini", fini)?;
    cx.export_function("new_instance", new_instance)?;
    cx.export_function("run", run)?;
    Ok(())
}
