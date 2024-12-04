# leptos-blog

## 开发步骤

安装trunk
```shell
cargo install trunk
```

安装tailwindcss4（目前4还在beta状态）
```shell
npm install tailwindcss@next @tailwindcss/postcss@next
```

保持tailwindcss的编译
```shell
npx @tailwindcss/cli -i style/app.css -o style/tailwind.css --watch
```

启动服务
```shell
trunk serve --open
```