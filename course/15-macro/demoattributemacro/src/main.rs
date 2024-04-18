/**
 * 属性宏
 *  属性宏定义了可附加到项的新外部属性，包括块中extern项、固有特征实现及特征定义。
 * 2024.04.18 by dralee
 */
extern crate myattrmacro;
use myattrmacro::show_streams;

#[show_streams]
fn invoke1(){}

#[show_streams(bar)]
fn invoke2(){}

#[show_streams(multiple=>tokens)]
fn invoke3(){}

#[show_streams{ delimiters }]
fn invoke4(){}

fn main() {
    invoke1();
    /*
     attr: ""
     item: "fn invoke1(){}" 
     */
    invoke2();
    /*
    attr: "bar"
    item: "fn invoke2(){}"
     */
    invoke3();
    /*
    attr: "multiple=>tokens"
    item: "fn invoke3(){}"
     */

    invoke4();
    /*
    attr: "delimiters"
    item: "fn invoke4(){}"
     */
}
