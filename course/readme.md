#### 创建
```shell
cargo new [projectname]
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
```
#### 创建lib
测试项目也可通过这种创建
```shell
cargo new [projectname] --lib
```

教材: https://doc.rust-lang.org/book/title-page.html
