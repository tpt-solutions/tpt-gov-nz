#!/usr/bin/env node
// Checks that the local toolchain and configuration are ready to run the platform.
// Usage: pnpm doctor
import { execFileSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const results = [];

function check(name, level, detail) {
  results.push({ name, level, detail });
}

function version(cmd, args = ["--version"]) {
  try {
    // On Windows, pnpm and similar tools are .cmd shims, which need a shell to run.
    return execFileSync(cmd, args, {
      encoding: "utf8",
      stdio: ["ignore", "pipe", "ignore"],
      shell: process.platform === "win32",
    }).trim();
  } catch {
    return null;
  }
}

// Node >= 20
const nodeMajor = Number(process.versions.node.split(".")[0]);
check("node >= 20", nodeMajor >= 20 ? "ok" : "fail", `found ${process.versions.node}`);

// pnpm >= 9
const pnpm = version("pnpm");
if (!pnpm) check("pnpm >= 9", "fail", "not found on PATH (install with `corepack enable`)");
else {
  const major = Number(pnpm.split(".")[0]);
  check("pnpm >= 9", major >= 9 ? "ok" : "fail", `found ${pnpm}`);
}

// cargo (Rust services)
const cargo = version("cargo");
check("cargo (Rust services)", cargo ? "ok" : "warn", cargo ?? "not found; Rust services cannot be built");

// docker (local stack)
const docker = version("docker");
check("docker (local stack)", docker ? "ok" : "warn", docker ?? "not found; `make stack` and `make demo` need Docker");

// .env
const envPath = join(root, ".env");
if (!existsSync(envPath)) {
  check(".env", "warn", "missing; copy .env.example to .env and fill in secrets");
} else {
  const env = readFileSync(envPath, "utf8");
  const required = ["TPT__GOV__SESSION_SECRET", "TPT__GOV__JWT_SECRET", "TPT__GOV__IDENTITY_API_KEY"];
  for (const key of required) {
    const m = env.match(new RegExp(`^${key}=(.*)$`, "m"));
    const value = m ? m[1].trim() : "";
    if (!value) check(`.env ${key}`, "warn", "not set; the service that needs it will refuse to start or fail closed");
    else if (/change-me|dev-insecure/i.test(value)) check(`.env ${key}`, "warn", "still a placeholder value");
    else check(`.env ${key}`, "ok", "set");
  }
  const issuer = env.match(/^TPT__GOV__ISSUER_PUBLIC_KEY=(.*)$/m);
  if (!issuer || !issuer[1].trim()) {
    check("TPT__GOV__ISSUER_PUBLIC_KEY", "warn", "not set; cross-department requests will be denied (fail closed)");
  } else {
    check("TPT__GOV__ISSUER_PUBLIC_KEY", "ok", "set");
  }
}

// node_modules
check(
  "pnpm install",
  existsSync(join(root, "node_modules")) ? "ok" : "warn",
  existsSync(join(root, "node_modules")) ? "dependencies installed" : "run `pnpm install`",
);

const icon = { ok: "PASS", warn: "WARN", fail: "FAIL" };
for (const r of results) console.log(`${icon[r.level]}  ${r.name}: ${r.detail}`);

const failed = results.filter((r) => r.level === "fail").length;
const warned = results.filter((r) => r.level === "warn").length;
console.log(`\n${results.length - failed - warned} passed, ${warned} warning(s), ${failed} failure(s).`);
process.exit(failed > 0 ? 1 : 0);
