use std::fs::File;
use std::io::Write;
use std::os::raw::c_void;
use std::path::Path;
use android_logger::{Config, FilterBuilder};
use jni::{JNIEnv, JavaVM, NativeMethod};
use jni::objects::{JObject, JString};
use jni::strings::JNIString;
use jni::sys::{jboolean, jint, jstring, JNI_FALSE, JNI_TRUE};
use log::{info, LevelFilter};
// use ndk::bitmap::Bitmap;

#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm: JavaVM, _reserved: *mut std::ffi::c_void) -> jint {
    // 获取 JNIEnv
    // GetEnv 是一个函数指针，用于从 JavaVM 获取 JNIEnv
    let mut env =   vm.get_env().unwrap() ;

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
    let methods = [
        NativeMethod {
            name : JNIString::from("callRustCode"), sig  : JNIString::from("()Ljava/lang/String;"), fn_ptr : call_rust_code as *mut c_void
        },
        NativeMethod {
            name : JNIString::from("equalTwoNum"), sig : JNIString::from("(II)Z"), fn_ptr : equal_two_num as *mut c_void
        },
        NativeMethod {
            name : JNIString::from("addFile"), sig : JNIString::from("(Ljava/lang/String;)Z"), fn_ptr : add_file as *mut c_void
        },
        // #[cfg(target_os = "android")]
        // NativeMethod {
        //     name : JNIString::from("getBitmapInfo"), sig : JNIString::from("(Landroid/graphics/Bitmap;)Ljava/lang/String;"), fn_ptr : get_bitmap_info as *mut c_void
        // },
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

#[no_mangle]
fn equal_two_num(_: JNIEnv, _: JObject, a: jint, b : jint ) -> jboolean {
    if a == b {
        JNI_TRUE
    } else {
        JNI_FALSE
    }
}

// #[no_mangle]
// unsafe fn get_bitmap_info(env: JNIEnv, _: JObject, src: jobject ) -> jstring {
//   let bitmap = Bitmap::from_jni(env.get_raw(), src);
//   let bitmap_info = bitmap.info().unwrap();
//   env.new_string(JNIString::from("{}")).unwrap().into_raw()
// }

#[no_mangle]
fn add_file(mut env: JNIEnv, _: JObject, path: JString) -> jboolean {
    let value =  env.get_string(&path).unwrap().into();
    add_file_internal(&value);
    JNI_TRUE
}

fn add_file_internal(path: &String) {
    match File::create(Path::new(path)) {
        Ok(mut file) => {
            let content = b"Hello, this is a file created by Rust!";
            if let Err(e) = file.write_all(content) {
                eprintln!("Failed to write to file: {}", e);
            } else {
                println!("File created and content written successfully!");
            }
        }
        Err(e) => {
            eprintln!("Failed to create file: {}", e);
        }
    }
}

#[cfg(test)]
pub mod test111 {

    #[test]
    pub fn test_2() {
      println!("Hello from rust test")
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



