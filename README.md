# atcoder-sample-downloder
atcoder-sample-downloder is command line tool for downloading sample tests cases of AtCoder problem.

## How to install
```
cargo install --git https://github.com/togatoga/atcoder-sample-downloder
```

## How to use
atcoder-sample-downloader has two main subcommands, `download` and `login`.

A subcommand `download` is downloading sample test cases of AtCoder problem from an AtCoder URL.
```
//download
$ atcoder-sample-downloder download https://atcoder.jp/contests/abc147/tasks/ab

```
A subcommand `login` is login AtCoder and save your cookie in your local.
```
$ atcoder-sample-downloder login
Please input Your username and password
Username >
Password >
SAVED YOUR COOKIE IN /Users/<YOUR NAME>/.atcoder-sample-downloader/cookie.jar
```

### NOTE
If you don't login atcoder, you can't access the running contest's problems.

i.e. you can't `download` sample test cases.
So, the cookie is used to get these problems.