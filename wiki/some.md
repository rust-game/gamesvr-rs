#关于设计的理论

# Context  (Session+Request+Response+errInfo)
贯穿一次请求的的上下文  
每次请求过来都会初始化一个全新的
一个全局的错误信息载体
#Session 
贯穿一次用户登录后的数据 -- 
这个数据不能直接保存在内存中 
考虑速度 感觉也不能写文件 这个是否需要有待确认 
设计出来 写端实现redis方式 确保速度

#Request
贯穿一次请求的的请求端的数据
实现从传输层读出数据  

#Response
贯穿一次请求的响应存储的内容
实现一个写数据到传输层

#Transfer
传输层

#Route
路由规则 对外只有唯一的接入口 ip:port 结构
通常是hash  [int]Action这种高效结构 客户端和服务端功能
一一对应的所以不需要复杂的路由保证速度
参数由内容携带 

#Action
 是一个请求的最小单位 通常处理一个请求的码
 
#Module 模块 
是一堆Action的集合  通常一类功能 比如登录模块的方法集合
单个服务(线程)的最小单位  
同一个Module可以运行到很多服务器 同一action转发需要Gate

#Server 
服务的最小单位
包含一个或者多个Module
唯一ip:port  需要向Gate注册 提供那些Action的服务

#Gate
记录Action 分散到那些Server上面
(怎么实现分布式互相发现，并做负载均衡这个问题还有待参考学习)
