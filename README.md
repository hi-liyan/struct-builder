# Struct builder

Rust 宏，用于自动为任意 struct 生成 builder pattern 代码。

## Usage

### 引入依赖
```toml
[dependencies]
struct-builder = { github = "https://github.com/hi-liyan/struct-builder" }
```

### 代码示例

在结构体添加 derive Builder 标签
```rust
#[derive(Builder)]
struct User {
    pub id: Option<u64>,
    pub name: Option<String>
}
```

使用 builder 函数构建结构体实例

```rust
fn main() {
    let user = User::builder()
        .id(Some(1))
        .name(Some("张三".to_string()))
        .build();
}
```

### 注意事项

由于我的使用场景比较局限，实现上并没有照顾其他情况，使用时需要注意以下问题：

1. 结构体的字段必须是 Option 类型，否则编译时报错。
2. builder 的 setter 函数接收的也是 Option 类型。
