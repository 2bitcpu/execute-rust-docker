### イメージの作成はこのディレクトリで
```
docker build . -t rust-executor
```

### 実行するときはどこでも良い
```
docker run --rm --mount type=bind,source="$(pwd)"/data,target=/data -t rust-executor -i /data/sample.xml -o /data/sample.json -w -p
```