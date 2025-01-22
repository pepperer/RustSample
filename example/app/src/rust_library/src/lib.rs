use android_logger::{Config, FilterBuilder};
use jni::JNIEnv;
use jni::objects::JObject;
use jni::sys::{JavaVM, jint, jstring};
use log::{info, trace, LevelFilter};

#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm: JavaVM, _reserved: *mut std::ffi::c_void) -> jint {
     // 在 JNI 环境中执行一些初始化操作（比如注册类，初始化资源等）
     println!("JNI_OnLoad called");
    android_logger::init_once(
        Config::default()
            .with_max_level(LevelFilter::Trace) // limit log level
            .with_tag("mytag") // logs will show under mytag tag
            .with_filter( // configure messages for specific crate
                FilterBuilder::new()
                    .parse("debug,hello::crate=error")
                    .build())
    );

    info!("JNI_OnLoad called");
    // 返回 JNI 的版本号
    jni::sys::JNI_VERSION_1_6
}
#[no_mangle]
extern fn Java_dev_matrix_rust_MainActivity_callRustCode(env: JNIEnv, _: JObject) -> jstring {
    env.new_string("Hello from rust112!").unwrap().into_raw()
}

#[cfg(test)] 
pub mod test111 {
    #[test]
    pub fn test_2() {
        println!("Hello from rust test!")
    }
}

pub mod lsd {
    pub fn print_lsd(){
        println!("Hello from rust lsd")
    }
}


pub fn test_android(){
    println!("Hello from rust test")
}

pub fn init_logcat() {
    android_logd_logger::builder()
    .parse_filters("debug")
    .tag("sss")
    .prepend_module(true)
    .init();
}

pub fn test_logcat_d() {
    android_logd_logger::write_event_now(1, "test lsd logcat ").unwrap();
}
