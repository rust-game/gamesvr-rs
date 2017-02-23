use std::collections::HashMap;
use {Action};

//! route 支持 map 一般游戏都是前后端固定的  支持自定义吧 暂时
pub trait Route {
    fn set<K,V:Action>(&mut self,key:K,value:V);
    fn get<K>(&self,key:K)->Action;
}

//简单路由 默认的
struct SimpleRoute{
    hash : HashMap
}

//实现简单的路由
impl Route for SimpleRoute{
    fn set(&mut self,key:u16,value:Action){
        self.hash.insert(key,value)
    }
    fn get(&self,key:u16)->Action{
        self.hash.get(key)
    }
}