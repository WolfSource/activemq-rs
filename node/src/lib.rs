#[macro_use]
extern crate lazy_static;

use neon::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;

struct AMQNode {
    ptr: activemq_rs::AmqObj,
    callback: Option<JsFunction>,
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
    let conn_type_index = cx.argument::<JsNumber>(0)?.value(&mut cx) as i32;
    let conn_type = match conn_type_index {
        0 => amq_connection_type::AMQ_CONSUMER,
        _ => amq_connection_type::AMQ_PRODUCER,
    };

    //create an amq instance, then box it
    let ins = activemq_rs::amq::new_instance(conn_type);
    let amq = AMQNode {
        ptr: ins,
        callback: None,
    };

    // lock the mutex and insert this activemq to it
    let mut map = MAPS.lock().unwrap();
    let index = map.len() as i32 + 1;
    map.insert(index, amq);

    Ok(cx.number(index))
}

pub fn run(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    // done
    let done: bool;
    // get unique index from argument
    let index = cx.argument::<JsNumber>(0)?.value(&mut cx) as i32;

    let map = MAPS.lock().unwrap();
    match map.get(&index) {
        Some(node) => {
            node.ptr.run();
            done = true;
        }
        _ => {
            done = false;
        }
    }

    Ok(cx.boolean(done))
}

pub fn send_message(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    // done
    let done: bool;
    // get unique index from argument
    let index = cx.argument::<JsNumber>(0)?.value(&mut cx) as i32;
    let msg = cx.argument::<JsString>(1)?.value(&mut cx);
    let priority = cx.argument::<JsNumber>(2)?.value(&mut cx) as i32;

    let map = MAPS.lock().unwrap();
    match map.get(&index) {
        Some(node) => {
            node.ptr.send_message(&msg, priority);
            done = true;
        }
        _ => {
            done = false;
        }
    }

    Ok(cx.boolean(done))
}

pub fn close(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    // done
    let done: bool;
    // get unique index from argument
    let index = cx.argument::<JsNumber>(0)?.value(&mut cx) as i32;

    let map = MAPS.lock().unwrap();
    match map.get(&index) {
        Some(node) => {
            node.ptr.close();
            done = true;
        }
        _ => {
            done = false;
        }
    }

    Ok(cx.boolean(done))
}

pub fn get_last_error(mut cx: FunctionContext) -> JsResult<JsString> {
    // done
    let error: String;
    // get unique index from argument
    let index = cx.argument::<JsNumber>(0)?.value(&mut cx) as i32;

    let map = MAPS.lock().unwrap();
    match map.get(&index) {
        Some(node) => {
            error = String::from(node.ptr.get_last_error());
        }
        _ => {
            error = String::from("");
        }
    }

    Ok(cx.string(error))
}

pub fn set_broker_uri(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    // done
    let done: bool;
    // get unique index from argument
    let index = cx.argument::<JsNumber>(0)?.value(&mut cx) as i32;
    let uri = cx.argument::<JsString>(1)?.value(&mut cx);

    let map = MAPS.lock().unwrap();
    match map.get(&index) {
        Some(node) => {
            node.ptr.set_broker_uri(&uri);
            done = true;
        }
        _ => {
            done = false;
        }
    }

    Ok(cx.boolean(done))
}

pub fn set_username(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    // done
    let done: bool;
    // get unique index from argument
    let index = cx.argument::<JsNumber>(0)?.value(&mut cx) as i32;
    let uri = cx.argument::<JsString>(1)?.value(&mut cx);

    let map = MAPS.lock().unwrap();
    match map.get(&index) {
        Some(node) => {
            node.ptr.set_username(&uri);
            done = true;
        }
        _ => {
            done = false;
        }
    }

    Ok(cx.boolean(done))
}

pub fn set_password(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    // done
    let done: bool;

    // get unique index from argument
    let index = cx.argument::<JsNumber>(0)?.value(&mut cx) as i32;
    let pass = cx.argument::<JsString>(1)?.value(&mut cx);

    let map = MAPS.lock().unwrap();
    match map.get(&index) {
        Some(node) => {
            node.ptr.set_password(&pass);
            done = true;
        }
        _ => {
            done = false;
        }
    }

    Ok(cx.boolean(done))
}

pub fn set_queue_or_topic_name(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    // done
    let done: bool;
    // get unique index from argument
    let index = cx.argument::<JsNumber>(0)?.value(&mut cx) as i32;
    let name = cx.argument::<JsString>(1)?.value(&mut cx);

    let map = MAPS.lock().unwrap();
    match map.get(&index) {
        Some(node) => {
            node.ptr.set_queue_or_topic_name(&name);
            done = true;
        }
        _ => {
            done = false;
        }
    }

    Ok(cx.boolean(done))
}

pub fn set_pipeline_type(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    // done
    let done: bool;

    // get unique index from argument
    let index = cx.argument::<JsNumber>(0)?.value(&mut cx) as i32;
    let t = cx.argument::<JsNumber>(1)?.value(&mut cx) as i32;
    let pipe_type = match t {
        0 => activemq_rs::amq::amq_pipeline_type::AMQ_QUEUE,
        _ => activemq_rs::amq::amq_pipeline_type::AMQ_TOPIC,
    };

    let map = MAPS.lock().unwrap();
    match map.get(&index) {
        Some(node) => {
            node.ptr.set_pipeline_type(pipe_type);
            done = true;
        }
        _ => {
            done = false;
        }
    }

    Ok(cx.boolean(done))
}

pub fn set_delivery_mode(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    // done
    let done: bool;
    // get unique index from argument
    let index = cx.argument::<JsNumber>(0)?.value(&mut cx) as i32;
    let mode = cx.argument::<JsNumber>(1)?.value(&mut cx) as i32;
    let del_mode = match mode {
        0 => activemq_rs::amq::amq_delivery_mode::AMQ_PERSISTENT,
        _ => activemq_rs::amq::amq_delivery_mode::AMQ_NON_PERSISTENT,
    };

    let map = MAPS.lock().unwrap();
    match map.get(&index) {
        Some(node) => {
            node.ptr.set_delivery_mode(del_mode);
            done = true;
        }
        _ => {
            done = false;
        }
    }

    Ok(cx.boolean(done))
}

pub fn set_session_transacted_mode(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    // done
    let done: bool;
    // get unique index from argument
    let index = cx.argument::<JsNumber>(0)?.value(&mut cx) as i32;
    let mode = cx.argument::<JsBoolean>(1)?.value(&mut cx);

    let map = MAPS.lock().unwrap();
    match map.get(&index) {
        Some(node) => {
            node.ptr.set_session_transacted_mode(mode);
            done = true;
        }
        _ => {
            done = false;
        }
    }

    Ok(cx.boolean(done))
}

pub fn set_on_message_recieved_callback(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    // done
    let done: bool;
    // get unique index from argument
    let index = cx.argument::<JsNumber>(0)?.value(&mut cx) as i32;
    let callback = cx.argument::<JsFunction>(1)?;

    let map = MAPS.lock().unwrap();
    match map.get(&index) {
        Some(node) => {
            // f.call(&mut cx, null, args)?
            //     .downcast::<JsNumber>()
            //     .or_throw(&mut cx);

            //let null = cx.null();
            //let arg = cx.null();
            //callback.call(&mut cx, null, arg)?;
            // node.ptr.set_on_message_recieved_callback(|x| -> () {
            //     //let arg = String::from("hello");
            //     let null = cx.null();
            //     callback.call(&mut cx, null)?;
            //     println!("{}", x);
            // });
            done = true;
        }
        _ => {
            done = false;
        }
    }

    Ok(cx.boolean(done))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("init", init)?;
    cx.export_function("fini", fini)?;
    cx.export_function("new_instance", new_instance)?;
    cx.export_function("run", run)?;
    cx.export_function("send_message", send_message)?;
    cx.export_function("close", close)?;
    cx.export_function("get_last_error", get_last_error)?;
    cx.export_function("set_broker_uri", set_broker_uri)?;
    cx.export_function("set_username", set_username)?;
    cx.export_function("set_password", set_password)?;
    cx.export_function("set_queue_or_topic_name", set_queue_or_topic_name)?;
    cx.export_function("set_pipeline_type", set_pipeline_type)?;
    cx.export_function("set_delivery_mode", set_delivery_mode)?;
    cx.export_function("set_session_transacted_mode", set_session_transacted_mode)?;
    cx.export_function(
        "set_on_message_recieved_callback",
        set_on_message_recieved_callback,
    )?;
    Ok(())
}
