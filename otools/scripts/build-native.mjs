import { copyFileSync, existsSync, mkdirSync } from "node:fs";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";
import { spawnSync } from "node:child_process";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, "..");
const nativeDir = path.join(rootDir, "native");
const outputDir = path.join(rootDir, "lib");
const profile = process.argv[2] === "release" ? "release" : "debug";

const artifactName = (() => {
  switch (process.platform) {
    case "darwin":
      return {
        source: "libcc_switch_native.dylib",
        target: "macOS.dylib",
      };
    case "win32":
      return {
        source: "cc_switch_native.dll",
        target: "Windows.dll",
      };
    default:
      return {
        source: "libcc_switch_native.so",
        target: "Linux.so",
      };
  }
})();

const cargoArgs = ["build"];
if (profile === "release") {
  cargoArgs.push("--release");
}

const cargo = spawnSync("cargo", cargoArgs, {
  cwd: nativeDir,
  stdio: "inherit",
});

if (cargo.status !== 0) {
  process.exit(cargo.status ?? 1);
}

const sourcePath = path.join(nativeDir, "target", profile, artifactName.source);
if (!existsSync(sourcePath)) {
  console.error(`Native artifact not found: ${sourcePath}`);
  process.exit(1);
}

mkdirSync(outputDir, { recursive: true });
const targetPath = path.join(outputDir, artifactName.target);
copyFileSync(sourcePath, targetPath);

console.log(`Copied native library to ${targetPath}`);
