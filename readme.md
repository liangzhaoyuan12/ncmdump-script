本程序是对ncmdump程序封装的脚本

转换源程序仓库：https://github.com/taurusxin/ncmdump

经过测试可以在Windows、Linux、MacOS上运行

支持系统和架构：
| 系统 | 架构 |
| ---- | ---- |
| Windows | x86_64 |
| Linux | x86_64 |
| MacOS | x86_64 |
| MacOS | arm64 |

目前已知问题：

在mac上调用系统API的选择文件的选择窗口上，会出现无法进入其他文件夹内选择文件的情况，但是如果通过搜索文件选择文件的话，是可以选中文件的。

解决方案：

可以通过搜索选择文件，或者输入文件路径进行转换

在Windows上不存在这个问题

在mac上调用系统API的选择文件夹的选择窗口上，不存在该问题

这个bug可能来自系统API的bug或者是调用的第三方库native-dialog = "0.9.0"的bug，目前找不到解决办法

若Linux请自行编译本程序