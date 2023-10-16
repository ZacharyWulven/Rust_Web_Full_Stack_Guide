use std::sync::Mutex;

pub struct AppState {
    // 响应字符串，这个字段共享于所有线程，初始化后它是一个不可变的
    pub health_check_response: String,
    /*
        也可以给每个线程共享，但它是可变的
        使用 Mutex 保证线程安全，即在修改数据前这个线程要先获取修改数据的控制权
     */
    pub visit_count: Mutex<u32>,
}