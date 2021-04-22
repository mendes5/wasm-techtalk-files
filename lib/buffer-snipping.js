const printVec2AtAddress = (buffer, address, numberFormatSize = 8) => {
  const start = address;
  const end = address + numberFormatSize * 2;

  const snipedVec3 = new DataView(buffer.slice(start, end));

  console.log(`Vec2@${address} {
  x: ${snipedVec3.getFloat64(0 * numberFormatSize, true)},
  y: ${snipedVec3.getFloat64(1 * numberFormatSize, true)},
}`);
};

const printVec3AtAddress = (buffer, address, numberFormatSize = 8) => {
  const start = address;
  const end = address + numberFormatSize * 3;

  const snipedVec3 = new DataView(buffer.slice(start, end));

  console.log(`Vec3@${address} {
  x: ${snipedVec3.getFloat64(0 * numberFormatSize, true)},
  y: ${snipedVec3.getFloat64(1 * numberFormatSize, true)},
  z: ${snipedVec3.getFloat64(2 * numberFormatSize, true)},
}`);
};
