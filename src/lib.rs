use duchess::prelude::*;
use once_cell::sync::OnceCell;
use std::thread;
use std::thread::JoinHandle;

duchess::java_package! {
    package java_test;

    public class Test {
        public int callback(java.lang.String);
        public java_test.Test testClone();
    }
}

static BACKGROUND_THREAD: OnceCell<JoinHandle<()>> = OnceCell::new();

#[duchess::java_function(java_test.Test::init_rust)]
fn init(test_instance: &java_test::Test) -> duchess::Result<Java<java::lang::String>> {
    let local_test_instance = test_instance.test_clone().execute()?.unwrap();
    BACKGROUND_THREAD.get_or_init(|| {
        thread::spawn(move || loop {
            local_test_instance.callback("rust to java".to_java()).execute();
            let one_sec = std::time::Duration::from_secs(1);
            thread::sleep(one_sec);
        })
    });
    Ok("".execute()?)
}
