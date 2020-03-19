[TOC]

# 一. CMakeLists.config用法
## 基本用法
### 指定include位置
- 语法: git_include { ... }
- 示例:
```cmake
IF (${OS} STREQUAL "WINDOWS")
    INCLUDE_DIRECTORIES(
        `git_include { target = win64 }`
    )
ELSEIF (${OS} STREQUAL "LINUX")
    INCLUDE_DIRECTORIES(
        `git_include { target = centos64 }`
    )
ENDIF()
```

### 指定静态库位置
- 语法: git_libpath { ... }
- 示例:
```cmake
IF (${OS} STREQUAL "WINDOWS")
    If (${FILE_PREFIX} STREQUAL "64")
        LINK_DIRECTORIES (
            `git_libpath { target = win64, platform = vs2015 }`
        )
    ELSE()
        LINK_DIRECTORIES (
            `git_libpath { target = win32, platform = vs2015 }`
        )
    ENDIF()
ELSEIF (${OS} STREQUAL "LINUX")
    LINK_DIRECTORIES (
        `git_libpath { target = centos64, platform = gcc }`
    )
ENDIF()
```

### 指定安装
- 描述
    - 根据LibraryConfig.toml中指定的安装, 循环写入
- 语法: git_bin_files_install { ... }
- 示例:
```cmake
`git_bin_files_install { target = win64, platform = vs2015, template = '
INSTALL (FILES "<0>" DESTINATION ${TARGET_INSTALL_DIR})
'}`
```
- 示例说明:
```
template: 指定循环写入的模板
<0>: 该语境下, 表示的是路径
```

### 导入 .cmake 文件
- 语法: git_cmakes(.cmake文件路径 ...)
- 示例:
```cmake
git_cmakes (
../../compile/cmakes/binary_general.cmake
)
```

### 自定义变量到.cmake文件中
- 描述
    - 设置的变量内容会替换.cmake中引用的位置
- 语法: set(变量名 变量值)
    - 如果变量的值是一个或者多个单一值, 直接 set(变量名 变量值1 变量值2)
    - 如果变量的值是一串, 使用闭合的 ^ 符号, 组合在一起
- 单一值示例:
```cmake
set (install_enable
    true
)
set (project_includes
    ..
    ${HEADER_DIR}/cbb38svrapi
    ${HEADER_DIR}/cbb95logicdata
)
```
- 串值示例:
```cmake
set (linux_install_files
^
COMMAND cp -f ${PROJECT_SOURCE_DIR}/implement/iotas_acd_hk/config/iotas_acd_hk.cfg ${TARGET_INSTALL_DIR}
^
)
```
- .cmake中引用自定义变量
```cmake
@{linux_install_files}
```

### 引入非cpp_store中库的方式
- 示例
```cmake
set(git_librarys
`{ name = iotas_acd_client, config = ..;..:include }`
`{ name = iotas_baseconfig, config = ../cbb95logicdata;../header/cbb95logicdata }`
`{ name = iotas_acd_client_db, config = ..;..:include }`
```
- 示例说明
```
name: 指定 CMakeLists.config 所在目录的名称
config: (如果是相对路径, 相对路径的起点是当前CMakeLists.config的位置)
    分号前: 指定库的 CMakeLists.config 的搜索路径
    分号后: 指定包含的路径
```

## 常见问题解决
### 依赖的库有多个, 但是只需要链接部分库
- 使用 subs 指定
- 示例
```cmake
set(git_librarys
    `{ name = opencv, version = 2.4.10, subs = ^~judge:"
            if var:'target' == str:'win64' {
                opencv_core2410,opencv_highgui2410,opencv_imgproc2410
            } elseif var:'target' == str:'win32' {
                opencv_core2410,opencv_highgui2410,opencv_imgproc2410
            } elseif var:'target' == str:'gnu64' {
                opencv_core,opencv_highgui,opencv_imgproc
            } else {
                opencv_core,opencv_highgui,opencv_imgproc
            }
        "~^ }`
)
```

# 二. LibraryConfig.toml用法
## 基本用法
### 版本属性
- 示例:
```toml
[package]
name = "algorithmtool"
authors = ''
edition = ''

[versions."0.1.0".attributes]
platform = "`var:'platform'`"
target = "`var:'target'`"
debug = "_d"
release = ""
rule = "$name$name_platform.$version$d_r"
includeRule = "`var:'config'`/`var:'version'`/include"
includeRules = ["`var:'config'`/`var:'version'`", "`var:'config'`/`var:'version'`/include/src", "`var:'config'`/`var:'version'`/include/src/postgres/include"]
libpathRule = "`var:'config'`/`var:'version'`/lib/`var:'platform'`_`var:'target'`"
libpathEnable = "false"
libnameEnable = "false"

[versions."0.1.0".attributes.map]
name_platform = """
        `judge:"
        if var:'target' == str:'win64' {
            64
        } elseif var:'target' == str:'win32' {
        } elseif var:'target' == str:'centos64' {
            64
        } elseif var:'target' == str:'ubuntu64' {
            64
        } else {
        }
        "`"""
```
- 示例说明
```toml
1. 指定版本
[versions."0.1.0".attributes]
上面的 0.1.0 表示的是版本

2. 引用CMakeLists.config中的变量
platform = "`var:'platform'`"
target = "`var:'target'`"
上面的 `var:'platform'` 就是CMakeLists.config中 git_include / git_libpath 指定的 platform 的值
`var:'target'` 同上

3. 使用 $引用toml块中的变量
rule = "$name.$version$d_r"
上面的 $version 就是版本值
$d_r 就是 debug / release 的值 ($d_r用于占位)

4. 指定自定义字段
[versions."0.1.0".attributes.map] 块下的k-v就是自定义的 key 和 value
可以在 rule 中使用, 如:
rule = "$name$name_platform.$version$d_r"
上面的 $name_platform 就是自定义的

5.1 include/libpath路径规则
如:
includeRule = "`var:'config'`/`var:'version'`/include"
libpathRule = "`var:'config'`/`var:'version'`/lib/`var:'platform'`_`var:'target'`"
其中:
`var:'config'` 指的是: LibraryConfig.toml 位置
`var:'version'` 就是版本号
上例中的 includeRule 的最终结果就是:
... algorithmtool/0.1.0/include
如果CMakeLists.config给定的 platform是 vs2015, target是win64, 那么 上例中的 libpathRule 的最终结果就是:
... algorithmtool/0.1.0/lib/vs2015_win64

5.2 配置多个包含路径
如:
includeRules = ["`var:'config'`/`var:'version'`", "`var:'config'`/`var:'version'`/include/src", "`var:'config'`/`var:'version'`/include/src/postgres/include"]

6. 指定依赖本库的上层是否需要包含本库
场景: 如果库只有头文件, 没有库, 就设置为 false
libpathEnable:
    值: true / false
    含义: 上层是否需要包含本层的库路径
libnameEnable:
    值: true / false
    含义: 上层是否需要引用本层的库
```

### 依赖配置
- 示例:
```toml
[versions."0.1.0"]
dependencies = [
    {name = "stringtool", version = "0.1.0", root = "..", includeEnable = "false"},
    {name = "pool", version = "0.1.0", root = "..", includeEnable = "false"},
    {name = "ospathtool", version = "0.1.0", root = "..", includeEnable = "false"},
    {name = "logictool", version = "0.1.0", root = "..", includeEnable = "false"},
    {name = "timetool", version = "0.1.0", root = ".."},
    {name = "curl", version = "7.55.1", root = "../../third", includeEnable = "false"},
    {name = "openssl", version = "1.1.0f", root = "../../third", enable = """
        `judge:"
        if var:'target' == str:'win64' {
            false
        } elseif var:'target' == str:'win32' {
            false
        } elseif var:'target' == str:'centos64' {
            true
        } elseif var:'target' == str:'ubuntu64' {
            true
        } else {
            true
        }
        "`""", includeEnable = "false"},
    {name = "zlib", version = "1.2.8", root = "../../third", includeEnable = "false"},
    {name = "libevent", version = "2.1.8", root = "../../third", subs = """
    `judge:"
        if var:'target' == str:'win64' {
            libevent, libevent_core, libevent_extras, libevent_openssl
        } elseif var:'target' == str:'win32' {
            libevent, libevent_core, libevent_extras, libevent_openssl
        } elseif var:'target' == str:'centos64' {
            event.`var:'version'`, event_core.`var:'version'`, event_extra.`var:'version'`, event_openssl.`var:'version'`, event_pthreads.`var:'version'`
        } elseif var:'target' == str:'ubuntu64' {
            event.`var:'version'`, event_core.`var:'version'`, event_extra.`var:'version'`, event_openssl.`var:'version'`, event_pthreads.`var:'version'`
        } else {
            event.`var:'version'`, event_core.`var:'version'`, event_extra.`var:'version'`, event_openssl.`var:'version'`, event_pthreads.`var:'version'`
        }
        "`
    """, includeEnable = "false"}
]
```
- 示例说明
```toml
1. name: 依赖库的名称 (toml中的 name 值)
2. version: 依赖库的版本号
3. root: 依赖库的搜索根路径
4. includeEnable: 决定上层是否需要包含本层依赖的下层库头文件目录 (支持 judge 语法)
5. enable: 决定上层是否需要连接本层依赖的下层库 (支持 judge 语法)
6. subs: 指定需要依赖的库名称 (支持 judge 语法)
```

### 本库提供多个库
- 示例:
```toml
[versions."1.15.2".attributes]
subs = "mongoc-1.0,bson-1.0"
includeSubs = "libbson-1.0,libmongoc-1.0"
```
- 示例说明:
```toml
1. subs: 指定调用本库时依赖的库名
2. includeSubs: 指定调用本库时需要包含的库目录
```
