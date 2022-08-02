# zjs - Yet Another JavaScript Runtime
Recently I saw a blog post which immediately caught my eye; ["Roll your own Javascript runtime"](https://deno.com/blog/roll-your-own-javascript-runtime) written by the folks over at Deno. Given the proliferation of Javascript runtimes over the past few years, I thought *"why not, I'll make my own Javascript runtime"*. The result is quite interesting, with ~100 lines of code we're able to run Javascript outside of a browser, and even send HTTP requests.

## Requirements
`z.js` requires `cargo` and `rustc` to be installed. The following versions are confirmed to work:
```
$ cargo -V
cargo 1.62.1 (a748cf5a3 2022-06-08)
$ rustc -V
rustc 1.62.1 (e092d0b6b 2022-07-16)
```
## Installation
If you want to try out `z.js` for yourself, follow the steps below:
```
# Clone the repository
git clone git@github.com:JLCarveth/zjs.git

cd zjs

cargo build
```
Once compiled, a `zjs` binary will be available at `target/debug/zjs`. You can run the example file with the following command:
```
target/debug/zjs example.js
```
The following should be output to the console:
```
[out]: "Hello, zjs!"
[err]: "Uh oh... Something went wrong."
[err]: "Unable to read file" "./log.txt" {}
[out]: "Read from a file" "./log.txt" "contents:" "I can write to a file."
[out]: "Removing file" "./log.txt"
[out]: "Removed file"
[out]: "Sending a GET HTTP Request"
[out]: "[\"0439f6cd-9a1a-412b-973f-3b3ba56ea0f9\"]"
```
