# Rust Box Demo

## 简介

演示 Rust Box 堆内存分配。

## 基本原理

Box<T> 在堆上存储数据，栈上存指针。用于递归类型和 trait 对象。

## 启动和使用

```bash
cargo run
```

## 教程

### 基本用法

```rust
let b = Box::new(5);
let s = Box::new(String::from("hello"));
```

### 递归类型

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

### Box::leak

```rust
let leak = Box::new(String::from("永远不会释放"));
let leaked: &'static str = Box::leak(leak);
```
