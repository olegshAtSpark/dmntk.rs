```
$ docker build -t dmntk:0.0.46-dev .
$ docker run --name dmntk -d -p 22022:22022 dmntk:0.0.46-dev
$ curl localhost:22022/system/info
```