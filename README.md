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

# config文件说明

本项目使用的滤镜配置文件的，是一个详细的大json。其内容是一个超大的map，key作为滤镜的别名，value则是她的具体实现细节。

```json
{
    "滤镜别名": [
        {"实现细节1号"},
        {"实现细节2号"},
        ...
        {"实现细节x号"}
    ]
}
```

一旦确定由`滤镜别名`实现滤镜效果，那么，程序会把图片交由`实现细节1号`处理，返回的结果再交由`实现细节2号`处理。最后，从`实现细节x号`的到最后的效果图。

目前的实现细节分为两种，基础滤镜，混合滤镜。事实上，这两者没有本质上的区别。

### basic filter

示例：

```json
[
    { "basic filter name1": "<args>" },
    { "basic filter name2": "<args>" },
    { "basic filter name3": "<args>" }
]
```

每一个`basic filter`和`blend filter`的使用都由一个单独的 json object（或者说map）定义。

### blend filter

// TODO

# 滤镜列表

| Name                | 参数                         | 说明                                |
| :------------------ | :--------------------------- | ----------------------------------- |
| brighten_by_percent | -100~100的浮点数             | 按百分比提高亮度                    |
| contrast            | -100~100的浮点数             | 按百分比提高对比度                  |
| fill_with_channels  | [r,g,b,a]，0~255的浮点数     | 使用颜色滤镜,`[255, 200, 200, 153]` |
| grayscale           | 无                           | 变成灰度图                          |
| huerotate           | 0～360整数(其实负数也可以啦) | 色相偏转                            |
| opaque              | 无                           | 设为不透明                          |
| saturate            | -100~100的浮点数             | 饱和度                              |
| sepia               | -100~100的浮点数             | 色温                                |

