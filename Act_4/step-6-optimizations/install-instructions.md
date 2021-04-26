#### Pass -O3 flag for maxium compile time optmization

```
cd ./nanosvg/example
emcc ./example2.c -I ../src -o ../../output/index.js -s EXTRA_EXPORTED_RUNTIME_METHODS='["cwrap"]' -O3
cd ../../output
http-server -o
```










































Fix memory leak.

Now you can use: `ALLOW_MEMORY_GROWTH`.

```
emcc ./example2.c -I ../src -o ../../output/index.html -s EXTRA_EXPORTED_RUNTIME_METHODS='["cwrap"]' -O3 -s ALLOW_MEMORY_GROWTH=1  
```
