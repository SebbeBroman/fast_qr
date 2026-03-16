import fs from "node:fs";
import path from "node:path";

function sourcePath() {
  switch (process.platform) {
    case "darwin":
      return path.resolve("target/release/libfast_qr.dylib");
    case "linux":
      return path.resolve("target/release/libfast_qr.so");
    case "win32":
      return path.resolve("target/release/fast_qr.dll");
    default:
      throw new Error(`Unsupported platform: ${process.platform}`);
  }
}

const source = sourcePath();
const destination = path.resolve("target/release/fast_qr.node");

fs.copyFileSync(source, destination);
console.log(`Prepared ${destination}`);
