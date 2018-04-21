# huahua

一个小滤镜，目前处于自娱自乐版。

本项目copy了很多[rustagram](https://github.com/ha-shine/rustagram)的源码，主要原因是这个项目的没有将大部分方法对外开放。



# Usage

目前只支持源码编译。

编译后可执行`./target/release/huahua -h`获取帮助：

```
huahua 0.1.0
Luke Euler <luke16times@gmail.com>
Apply custom filters to you photos

USAGE:
    huahua [OPTIONS] --filter <name> --input <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --filter <name>    Choose the filter in you filter file
    -c, --config <FILE>    Set the filter file [default: filter.json]
    -i, --input <FILE>     Path to the input image file
    -o, --output <FILE>    Output file name [default: output.jpg]
```

本项目也提供了一份默认的滤镜配置文件以供使用：`filter.json`。该配置文件基本是参考的instagram中的滤镜效果（仿自[rustagram](https://github.com/ha-shine/rustagram)）。如果想使用其他滤镜效果，可自行摸索😄。



如果仅仅想开箱即用，测试效果，可执行项目目录下的`test.sh`文件。当然，至少要保证你本体安装了**rust**，以及有一份**pics/test.jpg**作为滤镜效果的原图。