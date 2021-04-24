### After Crying:

We need to completelly change how Emscripten execution works.

The compiled program already wants a 23.svg on the file system, so we need to "write" this file first before running webassebly code.


First we need to stop running webassembly on page load, start by renamming `main` to something else (`runMain`) this will make Emscripten not to run the module.

Then add
```c
#include "emscripten.h"

EMSCRIPTEN_KEEPALIVE
```

TO the C file.

Nothing should execute now.

Now we need to manually run the main function. to do that we will need the `Module.cwrap` function from Emscripten. To enable this API add `-s EXTRA_EXPORTED_RUNTIME_METHODS='["cwrap"]'` to the build command.

```
cd ./nanosvg/example
emcc ./example2.c -I ../src -o ../../output/index.js -s EXTRA_EXPORTED_RUNTIME_METHODS='["cwrap"]'
```

Note that we changed the output to emit an js file instead, so we no longer get that Emscripten screen.
This also means we need to to create our own HTML file:

```html
<body>
  <script type="text/javascript" src="index.js"></script>
  <script>
    Module.onRuntimeInitialized  = () => {  
      const runMain = Module.cwrap("runMain", "number", []);
      runMain();
    };
  </script>
</body>
```

Now we should be getting the same error as before, but on the browser console.

```
cd ../../output
http-server -o
```

Now we can fetch a svg file and pass it to the virtual FileSystem

```html
  <script>
    Module.onRuntimeInitialized = () => {
      fetch("./23.svg")
        .then((res) => res.arrayBuffer())
        .then((buffer) => {
          const dataView = new Uint8Array(buffer);
          FS.writeFile("/23.svg", dataView); // The important part
        })
        .then(() => {
          const runMain = Module.cwrap("runMain", "number", []);
          runMain();
        });
    };
  </script>
```

Don't forget to update the data on the c file and to put the image on disk.

Running it should not give any errors.

Since we also write to file system we can now read it and create an image.

```html
  <script>
    Module.onRuntimeInitialized = () => {
      fetch("./23.svg")
        .then((res) => res.arrayBuffer())
        .then((buffer) => {
          const dataView = new Uint8Array(buffer);
          FS.writeFile("/23.svg", dataView);
        })
        .then(() => {
            const runMain = Module.cwrap("runMain", "number", []);
            runMain();

            // File was written on the virtal FS, now we read it
            // and create a image using browser APIs
            const fileData = FS.readFile("/svg.png");
            const blob = new Blob([fileData], { type: "image/png" });

            const image = new Image();
            image.src = URL.createObjectURL(blob);
            document.body.appendChild(image);
        });
    };
  </script>
```
