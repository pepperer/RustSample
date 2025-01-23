use std::os::raw::c_void;
use android_logger::{Config, FilterBuilder};
use jni::{JNIEnv, JavaVM, NativeMethod};
use jni::objects::{JClass, JObject};
use jni::strings::JNIString;
use jni::sys::{jint, jstring, JNINativeMethod};
use log::{info, trace, LevelFilter};

#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm: JavaVM, _reserved: *mut std::ffi::c_void) -> jint {
    // 获取 JNIEnv
    // GetEnv 是一个函数指针，用于从 JavaVM 获取 JNIEnv
    let mut env = unsafe { vm.get_env().unwrap() };

     // 在 JNI 环境中执行一些初始化操作（比如注册类，初始化资源等）
    println!("JNI_OnLoad called1");
    android_logger::init_once(
        Config::default()
            .with_max_level(LevelFilter::Trace) // limit log level
            .with_tag("lsd") // logs will show under mytag tag
            .with_filter( // configure messages for specific crate
                FilterBuilder::new()
                    .parse("debug,hello::crate=error")
                    .build())
    );

    info!("JNI_OnLoad called2");

    // 注册 native 方法
    let class_name = "dev/matrix/rust/MainActivity";
    let class = env.find_class(class_name).expect("Class not found");
    let methods = [
        NativeMethod {
            name : JNIString::from("callRustCode"),
            sig  : JNIString::from("()Ljava/lang/String;"),
            fn_ptr : call_rust_code as *mut c_void
        }
    ];

    env.register_native_methods(class_name, &methods).expect("Failed to register native methods");

    // 返回 JNI 的版本号
    jni::sys::JNI_VERSION_1_6
}

#[no_mangle]
fn call_rust_code(env: JNIEnv, _: JObject) -> jstring{
    info!("恭喜你成功实现的动态注册");
    env.new_string("Hello from rust112!").unwrap().into_raw()
}


// #[no_mangle]
// extern fn Java_dev_matrix_rust_MainActivity_callRustCode(env: JNIEnv, _: JObject) -> jstring {
//     env.new_string("Hello from rust112!").unwrap().into_raw()
// }
//
#[cfg(test)]
pub mod test111 {
    #[test]
    pub fn test_2() {
        println!("Hello from rust test!")
    }
}
//
// pub mod lsd {
//     pub fn print_lsd(){
//         println!("Hello from rust lsd")
//     }
// }
//
//
// pub fn test_android(){
//     println!("Hello from rust test")
// }
//
// pub fn init_logcat() {
//     android_logd_logger::builder()
//     .parse_filters("debug")
//     .tag("sss")
//     .prepend_module(true)
//     .init();
// }
//
// pub fn test_logcat_d() {
//     android_logd_logger::write_event_now(1, "test lsd logcat ").unwrap();
// }

/*
文档
static const JNINativeMethod nativeMethod[] = {
// Java中的函数名
{"stringFromJNI",
// 函数签名信息
"()Ljava/lang/String;",
// native的函数指针
(void *) (stringFromJNI)},
};
*/



