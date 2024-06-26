/* 
* 引用计数RefCell<T>智能指针,内部可变性
* 2024.04.09 by dralee
*/

pub trait Messager {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messager> {
    messager: &'a T,
    value: usize,
    max: usize
}

impl <'a, T> LimitTracker<'a, T> 
where T: Messager {
    pub fn new(messager:&'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker{
            messager,
            value: 0,
            max: max
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messager.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messager.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messager.send("warning: You've used up over 75% of your quota!");
        }
    }
}