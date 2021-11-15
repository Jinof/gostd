# [Gostd](https://github.com/wandercn/gostd)

[![crates.io](https://img.shields.io/crates/v/gostd.svg?color=yellow)](https://crates.io/crates/gostd)
[![Released API docs](https://docs.rs/gostd/badge.svg)](https://docs.rs/gostd)
[![GPL3 licensed](https://img.shields.io/github/license/wandercn/gostd.svg)](./LICENSE)
[![Downloads of Crates.io](https://img.shields.io/crates/d/gostd.svg)](https://crates.io/crates/gostd)
[![Lines of code](https://img.shields.io/tokei/lines/github/wandercn/gostd.svg)](#)
[![Build](https://img.shields.io/github/workflow/status/wandercn/gostd/Rust.svg)](#)
[![Languages](https://img.shields.io/github/languages/top/wandercn/gostd.svg)](#)

Gostd is the golang standard library implementation in rust-lang.

Gostd是rust-lang中的golang标准库实现。

rust 语法比go复杂，但是go代码简单好理解，想通过这个项目把go的标准库通过rust实现。以后有什么go的项目可以通过它方便翻译代码到rust，比如把 import "bytes" 改成 use gostd::bytes 就可以实现转换。

本项目纯粹个人兴趣，大概率会失败，但是梦想还是要有的万一它实现了。

go to rust,to be rust or to be failed.

# 已知难点
- go底层特性可能无法用rust实现，比如跟运行时相关的反射reflect，如果有依赖此库需要通过rust中的方式处理。
- 基础数据类型的转换，比如string是否采用自定义类型比如GoString来实现还是用string，float64(go) -> f64(rust)等等。
- 还有指针类型，幸好go的指针操作相比rust要少。
- go中的接口如何用rust的trait的实现？个人感觉还相对比较容易
- go中的不定参数在rust中只能用宏实现，这个比较麻烦，个人目前还不知道宏能不能像方法一样绑定到sturct上。

## 需要用rust实现的go标准库列表，go.1.17.1代码做参考。

go 这个包不会实现，因为我们转换成rust基本用不上这个包。
```
├── archive
├── bufio
├── builtin
├── bytes
├── cmd
├── compress
├── container
├── context
├── crypto
├── database
├── debug
├── embed
├── encoding
├── errors
├── expvar
├── flag
├── fmt
├── go
├── hash
├── html
├── image
├── index
├── internal
├── io
├── log
├── math
├── mime
├── net
├── os
├── path
├── plugin
├── reflect
├── regexp
├── runtime
├── sort
├── strconv
├── strings
├── sync
├── syscall
├── testdata
├── testing
├── text
├── time
├── unicode
├── unsafe
└── vendor
```

## 对应预期实现后的gostd的Model列表
```
use gostd::archive
use gostd::bufio
use gostd::builtin
use gostd::bytes
use gostd::cmd
use gostd::compress
use gostd::container
use gostd::context
use gostd::crypto
use gostd::database
use gostd::debug
use gostd::embed
use gostd::encoding
use gostd::errors
use gostd::expvar
use gostd::flag
use gostd::fmt
use gostd::go
use gostd::hash
use gostd::html
use gostd::image
use gostd::index
use gostd::internal
use gostd::io
use gostd::log
use gostd::math
use gostd::mime
use gostd::net
use gostd::os
use gostd::path
use gostd::plugin
use gostd::reflect
use gostd::regexp
use gostd::runtime
use gostd::sort
use gostd::strconv
use gostd::strings
use gostd::sync
use gostd::syscall
use gostd::testdata
use gostd::testing
use gostd::text
use gostd::time
use gostd::unicode
use gostd::unsafe
use gostd::vendor
```

# 大致方向

- 分析go标准库的依赖情况，从最底层的库开始实现。
- go源码中的单元测试也会一并用rust实现，这样可以保证代码的健壮性。

# todo

- [x] Go基础类型在rust实现，在gostd::builtin 中,比如 int64 = i64, int32 = i32
- [x] 强制转换宏,例如 2 as i64 等价 int64!(2) 跟Go的int64(2)就多个！
- [x] time库在rust实现 gostd::time
- [x] time库支持macOSX 和linux平台，通过libc库调用C函数实现 time::Now()
- [x] time，支持各种格式显示时间。
- [x] docs.rs文档增加例子程序"RUN"按钮,但是要复制代码本地运行,在rust play运行不了(因为下载量没到前100)。
- [x] time支持local时区信息自动从系统读取，可以用time::Now()获取本地时间。

# 独立发布包
 
独立发布gostd_time，代码等价于 use gostd::time 。 

独立发布gostd_builtin, 代码等价于 use gostd::builtin 。 