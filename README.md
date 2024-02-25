# thunder-book-visualizer

thunder 本に出てくるゲームのビジュアライザ

## ローカルでの開発

- 参考：https://dioxuslabs.com/learn/0.4/cookbook/tailwind
- Run the following command in the root of the project to start the tailwind css compiler:

```bash
npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch
```

### Web

- Run the following command in the root of the project to start the dioxus dev server:

```bash
dx serve --hot-reload
```

- Open the browser to http://localhost:8080.

## リリース方法

1. リリースビルドを行う

```bash
dx build --release
```

2. git commit & push を行う
