Esse código deve dar erro de compilação: 

```rust
fn main() {
    let mut s1 = String::from("Texto");
    let s2 = &s1;
    s1.push_str("!");
    println!("{}",s2);
}
```

Mas se você comentar a última linha, ele passa na compilação: 

```rust
fn main() {
    let mut s1 = String::from("Texto");
    let s2 = &s1;
    s1.push_str("!");
    //println!("{}",s2);
}
```


