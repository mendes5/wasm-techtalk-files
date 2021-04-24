### 1. Compile

```
emcc ./main.c
```

### 2. Create an index.html file to view in browser and serve
index.html:

```html
<script src='./a.out.js'></script>
```

```
http-server -o
```

Alternatively use nodejs

```
node ./a.out.js
```
