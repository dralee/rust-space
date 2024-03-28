/**
 * 重复代码去除,泛型
 * 2024.03.28 by dralee
 */

/**
 * 相同内容代码,重复性,抽取为方法
 */
pub fn duplication_demo() {
    let number_list = vec![12,50,62,30,100,82];
    let mut largest = &number_list[0];
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);

    let number_list = vec![212,50,1025,62,30,100,82,99];
    let mut largest = &number_list[0];
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);
}

// 简化重复代码
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest { 
            largest = item;
        }
    }

    largest
}

pub fn remove_duplication_demo() {
    println!("remove duplication code:");
    let number_list = vec![12,50,62,30,100,82];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    
    let number_list = vec![212,50,1025,62,30,100,82,99];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
}

// 适应多种类型数据,使用泛型
fn largest_g<T>(list: &[T]) -> &T where T : core::cmp::PartialOrd {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl <T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    // 到原点距离
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt() // x^2+y^2开方 
    }
}

#[derive(Debug)]
struct Point2<T,U> {
    x:T,
    y:U
}

#[derive(Debug)]
struct Point3<X1,Y1>{
    x: X1,
    y: Y1,
}
impl <X1,Y1> Point3<X1,Y1> {
    // 混合类型
    fn mixup<X2,Y2>(self, other: Point3<X2, Y2>) -> Point3<X1,Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn generic_fn(){
    println!("generic function:");
    let number_list = vec![12,50,62,30,100,82];
    let result = largest_g(&number_list);
    println!("The largest number is {}", result);
    
    let char_list = vec!['h','e','l','l','o',' ','w','o','r','l','d'];
    let result = largest_g(&char_list);
    println!("The largest char is {}", result);

    let int_point = Point{ x: 5, y: 10};
    let float_point = Point { x: 12.5, y:18.2};
    println!("int: {:?}, float: {:?}", int_point, float_point);
    println!("int_point.x = {}", int_point.x());
    println!("distnace from origin: {}", float_point.distance_from_origin());
    
    let int_float_point = Point2 { x: 10, y:18.2};
    println!("int float: {:?}", int_float_point);

    let p1 = Point3{ x: 5, y : 10.5};
    let p2 = Point3{x: "Hello", y: 'g'};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}