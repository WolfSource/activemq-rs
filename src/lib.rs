#[cxx::bridge(namespace = "amq")]
pub mod amq {
    enum amq_connection_type {
        AMQ_CONSUMER = 0,
        AMQ_PRODUCER = 1,
    }
    enum amq_pipeline_type {
        AMQ_QUEUE = 0,
        AMQ_TOPIC = 1,
    }
    enum amq_delivery_mode {
        AMQ_PERSISTENT = 0,
        AMQ_NON_PERSISTENT = 1,
    }

    unsafe extern "C++" {

        include!("activemq-rs/src/cxx/inlines.h");

        pub fn init() -> ();
        pub fn fini() -> ();

        include!("activemq-rs/src/cxx/producer_consumer.h");

        type producer_consumer; //for class

        pub fn run(&self) -> ();
        pub fn send_message(&self, pMessage: &str, pPriority: i32) -> ();
        pub fn close(&self) -> ();

        pub fn get_last_error(&self) -> &str;

        pub fn set_broker_uri(&self, p: &str) -> ();
        pub fn set_username(&self, p: &str) -> ();
        pub fn set_password(&self, p: &str) -> ();
        pub fn set_queue_or_topic_name(&self, p: &str) -> ();
        pub fn set_pipeline_type(&self, p: amq_pipeline_type) -> ();
        pub fn set_delivery_mode(&self, p: amq_delivery_mode) -> ();
        pub fn set_session_transacted_mode(&self, p: bool) -> ();
        pub fn set_on_message_recieved_callback(&self, f: fn(String) -> ()) -> ();

        pub fn new_instance(p: amq_connection_type) -> UniquePtr<producer_consumer>;
    }
}

pub type AmqObj = cxx::UniquePtr<amq::producer_consumer>;

fn fn_callback(str: String) -> () {
    println!("{}", str);
}

#[test]
fn tests() -> () {
    amq::init();
    let _p: AmqObj = amq::new_instance(amq::amq_connection_type::AMQ_CONSUMER);
    _p.set_on_message_recieved_callback(fn_callback);
    amq::fini();
}
