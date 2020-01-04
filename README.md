
```
cargo new hello_rust --bin
cd hello_rust
git init
git add .
git commit -m “cargo new hello_rust —bin”

cargo run
```

output:
```
   Compiling hello_rust v0.1.0 (/Users/sallen/src/rust/hello_rust)
    Finished dev [unoptimized + debuginfo] target(s) in 1.47s
     Running `target/debug/hello_rust`
Hello, world!
```

```
heroku create --buildpack emk/rust
```

output:
```
Creating app... done, ⬢ peaceful-gorge-05620
Setting buildpack to emk/rust... done
https://peaceful-gorge-05620.herokuapp.com/ | https://git.heroku.com/peaceful-gorge-05620.git
```