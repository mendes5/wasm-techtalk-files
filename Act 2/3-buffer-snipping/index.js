fetch("../out/main.wasm")
  .then((response) => response.arrayBuffer())
  .then((bytes) =>
    WebAssembly.instantiate(bytes, {
      env: {
        __syscall1: (...args) => console.log('__syscall1', args),
        __syscall2: (...args) => console.log('__syscall2', args),
        __syscall3: (...args) => console.log('__syscall3', args),
        __syscall4: (...args) => console.log('__syscall4', args),
        __syscall5: (...args) => console.log('__syscall5', args),
        __syscall6: (...args) => console.log('__syscall6', args),
      },
    })
  )
  .then(({ instance }) => {
      const result = instance.exports.vec2(10.65, 0.5);
      console.log({ result });
    
      const buffer = instance.exports.memory.buffer;

      const numberFormatSize = 8; // sizeof(double)
      const startAddress = result;
      const endAddress = startAddress + numberFormatSize * 2;
    
      const memorySectionCopy = new DataView(buffer.slice(startAddress, endAddress));
    
      const xComponent = memorySectionCopy.getFloat64(0 * numberFormatSize, true);
      const yComponent = memorySectionCopy.getFloat64(1 * numberFormatSize, true);
              
      console.log({ xComponent, yComponent });
  })
  .catch(console.error);
