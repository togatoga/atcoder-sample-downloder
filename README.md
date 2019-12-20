# atcoder-sample-downloder
atcoder-sample-downloder is command line tool for downloading sample tests cases of AtCoder problem.

## How to install
```
cargo install --git https://github.com/togatoga/atcoder-sample-downloder
```

## How to use
atcoder-sample-downloader has two main subcommands, `download` and `login`.

A subcommand `download` is to download sample test cases of AtCoder problem from an AtCoder URL.
```
$ atcoder-sample-downloder download https://atcoder.jp/contests/abc147/tasks/abc147_a
====== Download Result ======
=== Sample Test Case 1 ===
Input:
5 7 9

Output:
win

=== Sample Test Case 2 ===
Input:
13 7 2

Output:
bust

=============================
$ bat sample_*
───────┬──────────────────────────────────────────────────────────────────────────────────────────────────────────────
       │ File: sample_input_1.txt
───────┼──────────────────────────────────────────────────────────────────────────────────────────────────────────────
   1   │ 5 7 9
───────┴──────────────────────────────────────────────────────────────────────────────────────────────────────────────
───────┬──────────────────────────────────────────────────────────────────────────────────────────────────────────────
       │ File: sample_input_2.txt
───────┼──────────────────────────────────────────────────────────────────────────────────────────────────────────────
   1   │ 13 7 2
───────┴──────────────────────────────────────────────────────────────────────────────────────────────────────────────
───────┬──────────────────────────────────────────────────────────────────────────────────────────────────────────────
       │ File: sample_output_1.txt
───────┼──────────────────────────────────────────────────────────────────────────────────────────────────────────────
   1   │ win
───────┴──────────────────────────────────────────────────────────────────────────────────────────────────────────────
───────┬──────────────────────────────────────────────────────────────────────────────────────────────────────────────
       │ File: sample_output_2.txt
───────┼──────────────────────────────────────────────────────────────────────────────────────────────────────────────
   1   │ bust
───────┴──────────────────────────────────────────────────────────────────────────────────────────────────────────────

```
A subcommand `login` is to login AtCoder and save your cookie in your local.
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
