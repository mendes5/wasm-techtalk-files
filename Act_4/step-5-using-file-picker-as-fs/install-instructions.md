```
cd ./nanosvg/example
emcc ./example2.c -I ../src -o ../../output/index.js -s EXTRA_EXPORTED_RUNTIME_METHODS='["cwrap"]'
cd ../../output
http-server -o
```

Change the HTML to:

```html
<input type="file" id="file-selector" />

<body>
  <script type="text/javascript" src="index.js"></script>
  <script>
    const fileSelector = document.querySelector("#file-selector");

    Module.onRuntimeInitialized = () => {
      fileSelector.addEventListener("change", function name(event) {
        const file = event.target.files.item(0);

        file.arrayBuffer().then((buffer) => {
          const dataView = new Uint8Array(buffer);
          FS.writeFile("/23.svg", dataView);

          const runMain = Module.cwrap("runMain", "number", []);
          runMain();

          const fileData = FS.readFile("/svg.png");

          const blob = new Blob([fileData], { type: "image/png" });
          const image = new Image();
          image.src = URL.createObjectURL(blob);
          document.body.appendChild(image);
        });
      });
    };
  </script>
</body>
```