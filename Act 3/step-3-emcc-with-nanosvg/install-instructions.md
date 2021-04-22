### 1. Make sure the Emscripten toolchain is running

```
emcc
```
Should log `emcc: error: no input files`

### 2 On the example folder run

```
cd nanosvg/example
emcc ./example2.c -I ../src -o ../../output/index.html
```
* `-I ../src` same as before
* `-o ../../output.index.html` specify output folder and filename

### 3 On the output folder urn

```
cd ../../output
http-server -o
```