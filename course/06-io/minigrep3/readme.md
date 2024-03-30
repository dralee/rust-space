### 查询文本文件中包含指定内容的行内容
```bash
$ cargo run -- the poem.txt
Then there's a pair of us - don't tell!
To tell your name the livelong day
```
忽略大小写查询
```bash
IGNORE_CASE=1 cargo run -- bOdy  poem.txt
```
powershell中设置环境变量
```powershell
PS> $Env:IGNORE_CASE=1;cargo run -- bOdy  poem.txt
PS> Remove-Item Env:IGNORE_CASE  # 移除环境变量IGNORE_CASE
```
错误输出
```bash
cargo run > output.txt
```
```bash
cargo run -- to poem.txt > output.txt
```
