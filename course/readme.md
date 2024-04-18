#### 创建
```shell
cargo new [projectname]
```

### 检测编译
```shell
cargo check
```

#### 编译
```shell
cd [projectname]
cargo build
```

#### 运行
```shell
cd [projectname]
cargo run
```
```bash
cargo run -p 项目名  # 指定项目名进行运行
```
#### 测试
```shell
cargo test

cargo test --help # 显示test的参数
cargo test -- --help # 显示分隔符之后可以使用的选项。
cargo test -- --test-threads=1  # 指定同时只运行一个测试,防止同时运行有些测试需要写数据等导致错误
cargo test -- --show-output # 如果我们还希望看到通过测试的打印值，我们可以告诉 Rust 也使用—— show-output 显示成功测试的输出。
cargo test 名称 # 指定测试名称,只运行指定的测试(名称即方法名),如果只是名称的一部分,则多个方法含有指定名称的都会执行
cargo test -- --ignored # 只有打上#[ignore]的测试会运行
cargo test -- --include-ignored # 包含有打上#[ignore]的也运行,没打的也运行
cargo test --test 文件名 # 通过指定文件名测试
cargo test -p 项目名 # 测试指定项目
```
#### 创建lib
测试项目也可通过这种创建
```shell
cargo new [projectname] --lib
```
发布
```shell
cargo build --release
```
配置,按不同环境设置配置参数
```xml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```
#### 登录cargo.io
到https://crates.io/中注册登录,通过github账号登录,跳转到https://crates.io/me/获取api密钥
```bash
 cargo login abcdefghijklmnopqrstuvwxyz012345
 # 会存入~/.cargo/credentials中
```
发布crate包设置, Cargo.toml中
```bash
[package]
name = "包名"   # 包名需要保持与crates.io中唯一不重复
license = "MIT" # 许可证类型声明  http://spdx.org/licenses/
```
* 示例
```bash
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"
```
执行发包
```bash
cargo publish  # 会推送到crate.io上
```
注: 发布是永久性的,版本永远无法覆盖,代码也无法删除
虽然无法删除,但可让其失效
```bash
cargo yank --vers 1.0.1 # 指定拉取1.0.1版本
cargo yank --vers 1.0.1 --undo # 撤消指定拉取1.0.1版本
```
安装
```bash
cargo install xxx # 安装,如 cargo install ripgrep
# 安装到$HOME/.cargo/bin中
```
### 智能指针
* Box<T> 用于堆上分配值
	* 当有一类型,其大小编译时无法知道,并且希望在需要确切大小的上下文中使用该类型的值时;
	* 当拥有大量数据并且想要转让所有权,但确保在这样做时不会复制数据;
	* 当想拥有一个值,并且只关心它是一种实现特定特征类型,而不是属于特定类型时
* Rc<T> 一种支持多重所有权的引用计数类型
* Ref<T>和RefMut<T>,通过RefCell<T>访问,一种在运行时而不是编译时强制执行借用规则的类型
#### Deref
* 从&T到&U when T: Deref<Target=U>
* 从&mut T到&mut U when T:DerefMut<Target=U>
* 从&mut T到&U when T:Deref<Target=U>


教材: https://doc.rust-lang.org/book/title-page.html
