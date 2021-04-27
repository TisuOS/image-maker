# Image Maker
用于快速将指定文件夹打包成镜像，内部的文件系统默认是 [TianMu](https://github.com/TisuOS/tianmu-fs)。如果你想要定制自己的文件系统格式，请实现 `src/require.rs` 下的 trait

Image Maker is for packing the specific folder in the format [TianMu](https://github.com/TisuOS/tianmu-fs). If you want to custom you own fs format, realize the trait in the `src/require.rs`

## 用法(Usage)
```
cargo run test/ img
```