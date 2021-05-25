use std::env;

fn main() {
    //get profile
    let profile = env::var("PROFILE").unwrap();

    // get activemq deps path
    let current_path = match env::current_dir() {
        Ok(x) => x.into_os_string().into_string().unwrap(),
        Err(_) => "".to_string(),
    };
    let activemq_path = current_path + "/deps/activemq-cpp/";

    // set cxx build
    let mut build = cxx_build::bridge("src/lib.rs");

    let target_os = env::var("CARGO_CFG_TARGET_OS");
    match target_os.as_ref().map(|x| &**x) {
        Ok("linux") | Ok("android") => {}
        Ok("windows") => {
            //link the necessery C/C++ libraries
            build.define("WIN32", "WIN32");
            build.define("_WINDOWS", "_WINDOWS");

            if profile == "debug" {
                build.define("DEBUG", "DEBUG");
                println!("cargo:rustc-link-lib=msvcrtd");
            } else {
                println!("cargo:rustc-link-lib=msvcrt");
            }
            println!("cargo:rustc-link-lib=dylib=Shell32");
            println!("cargo:rustc-link-lib=dylib=Rpcrt4");
            println!("cargo:rustc-link-lib=dylib=Mswsock");
            println!("cargo:rustc-link-lib=static=libactivemq-cpp");
        }
        os => panic!("unknown target os {:?}!", os),
    }

    println!(
        "cargo:rustc-link-search=native={}/lib/{}/{}/{}",
        activemq_path,
        std::env::consts::OS,
        std::env::consts::ARCH,
        profile
    );

    build.include(activemq_path + "/include");
    build.file("src/cxx/producer_consumer.cpp");
    build.flag_if_supported("-std=c++20");
    build.compile("activemq-rs");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/cxx/inlines.h");
    println!("cargo:rerun-if-changed=src/cxx/producer_consumer.cpp");
    println!("cargo:rerun-if-changed=src/cxx/producer_consumer.h");
}
