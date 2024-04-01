
/**
 * 比较循环跟迭代器性能
 * 2024.04.01 by dralee
 */
fn main() {
    demo();
}

fn demo(){
    let buffer: &mut [i32] = &mut [123,456,78,21,33,11,34,89,123,890,44,23,99,78,102,92,790,810,77,912,125,1024];
    let coefficients: [i64; 12] = [1,2,3,4,5,6,7,8,9,10,11,12];
    let qlp_shift: i16 = 5;

    // for i in 2..coefficients.len() {
    //     println!("{i}={}", coefficients[i]);
    // }
    println!("{:?}", coefficients);
    for i in 12..buffer.len() {
        let prediction = coefficients.iter()
                .zip(&buffer[i-12..i])
                .map(|(&c, &s)| c * s as i64)
                .sum::<i64>() >> qlp_shift;
        
        let delta = buffer[i];
        buffer[i] = prediction as i32 + delta;
    }

    println!("buffer:{:?}", buffer);
}