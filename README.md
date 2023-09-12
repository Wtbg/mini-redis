# README

## Introduce

本项目为浙江大学短学期Rust课程大作业，mini-redis的开发的前置项目，一个极简的mini-redis，支持一些基础命令（见Feature模块），并包装了命令行应用。

## Features

本项目支持命令如下：(可在运行客户端后使用--help查看)

![features.png](https://img1.imgtp.com/2023/09/12/0E5Ou5c1.png)

## How To Use

使用在项目文件夹使用cargo run --bin server运行服务端程序

使用在项目文件夹使用cargo run -bin client运行客户端程序，如果需要单独运行可执行文件，可执行cargo build执行，在对应文件夹找到可执行文件复制到需要执行的位置，或将可执行文件目录加入环境变量。

1. ping
   **用法**：
   ![ping_h.png](https://img1.imgtp.com/2023/09/12/lP5ddR7q.png)
   **使用效果**：在第一次使用ping后关闭服务器，已表现出ping的效果
   ![ping_use.png](https://img1.imgtp.com/2023/09/12/ZKK9UmlY.png)
   ping命令用于检测和服务器的连通性，正常返回是Ok，连接失败返回具体错误信息

2. set & get
   用法：
   ![set_h.png](https://img1.imgtp.com/2023/09/12/pfQ3TkWc.png)
   ![get_h.png](https://img1.imgtp.com/2023/09/12/eJOzQLuZ.png)
   使用效果：左右分别为两终端开启的client
   ![set_use.png](https://img1.imgtp.com/2023/09/12/pRCuCUA7.png)

> 多个连接到同一个redis的客户端共享一个数据库，可跨客户端共享键值对

> 当键存在时，命令行回显对应的值，若不存在则显示(nil)

> 存储存在的键值对会进行覆盖

![mult_set.png](https://img1.imgtp.com/2023/09/12/RT0oCYfe.png)

3. del
   用法：
   ![del_h.png](https://img1.imgtp.com/2023/09/12/Ea1mtDYV.png)
   使用效果：左右分别为两终端开启的client
   ![del_use.png](https://img1.imgtp.com/2023/09/12/YCCl1Mw7.png)

> del可以删除一个键值对
> 
> del执行会回显操作影响键值对数量，删除成功返回(integer)1，不存在返回(integer)0

4. subscribe & publish
   用法：
   ![subscrible_h.png](https://img1.imgtp.com/2023/09/12/SHgru8bz.png)
   ![publish_h.png](https://img1.imgtp.com/2023/09/12/hrZDDDhP.png)
   使用效果：左右分别为两终端开启的client
   ![subscribe.png](https://img1.imgtp.com/2023/09/12/hR8W9oUE.png)

> subscribe后会陷入循环，不再响应命令以防和输出混淆，该channel的信息会一行行显示在命令行中如图所示
> publish会将信息添加到channel
> subscribe只会显示订阅后被publish的消息，即不会显示历史信息

## Other

由于使用了clap crate包装命令行应用，用户体验较好，可以使用--help/-h查看帮助信息，命令输入错误可以看到相关提示。

## Beg Score

亲爱的助教哥哥，初次接触RPC写这样一个项目难度还是有一些的，对于作业要求的特性我都实现了，进阶和基础功能都完成了，希望能给高点分，给跪了Orz
