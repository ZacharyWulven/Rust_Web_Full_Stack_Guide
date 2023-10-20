use core::num;
use std::num::ParseIntError;

fn main() {
    // let result = square("25");
    let result = square("Rt");

    println!("{:?}", result);
}


// 这就是一个可能出错的函数
fn square(val: &str) -> Result<i32, ParseIntError> {
    /*
        如果成功获取 i32，则 num 的值就是其值，
        否则此函数直接返回了，反正的就是 Err(e)
     */
    let num = val.parse::<i32>()?;
    Ok(num ^ 2)

    // match val.parse::<i32>() {
    //     // 能成功解析成一个整数，就返回这个整数的平方
    //     Ok(num) => Ok(num.pow(2)),
    //     Err(e) => Err(e),
    // }
}

// 这就是一个可能出错的函数
// fn square(val: &str) -> Result<i32, ParseIntError> {
//     match val.parse::<i32>() {
//         // 能成功解析成一个整数，就返回这个整数的平方
//         Ok(num) => Ok(num.pow(2)),
//         Err(e) => Err(e),
//     }
// }

