use neon::prelude::*;

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

fn init(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    activemq_rs::amq::init();
    Ok(cx.boolean(true))
}

fn fini(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    activemq_rs::amq::fini();
    Ok(cx.boolean(true))
}

fn new_instance(mut cx: FunctionContext) -> JsResult<JsBox<AMQNode>> {
    use activemq_rs::amq::*;

    let conn_type_index = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let conn_type: amq_connection_type = match conn_type_index {
        0.0 => amq_connection_type::AMQ_CONSUMER,
        _ => amq_connection_type::AMQ_PRODUCER,
    };

    let ins = activemq_rs::amq::new_instance(conn_type);
    let ins_boxed = Box::new(ins);
    let amq = AMQNode {
        ptr: Box::into_raw(ins_boxed) as *mut std::os::raw::c_void,
    };
    Ok(cx.boxed(amq))
}

pub fn run(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    use activemq_rs::amq::*;

    // Get the `this` value as a `JsBox<AMQNode>`
    let js = cx.this().downcast_or_throw::<JsBox<AMQNode>, _>(&mut cx)?;
    let amq_node = unsafe { Box::from_raw(js.ptr as *mut activemq_rs::AmqObj) };
    amq_node.run();
    Ok(cx.boolean(true))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("init", init)?;
    cx.export_function("fini", fini)?;
    cx.export_function("new_instance", new_instance)?;
    cx.export_function("run", run)?;
    Ok(())
}
