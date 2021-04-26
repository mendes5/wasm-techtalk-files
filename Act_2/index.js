fetch("../out/main.wasm")
  .then((response) => response.arrayBuffer())
  .then((bytes) => WebAssembly.instantiate(bytes))
  .then(({ instance }) => {
      const result = instance.exports.fn();
      console.log({ result });
  })
  .catch(console.error);
