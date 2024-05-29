// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.
import { core, internals, primordials } from "ext:core/mod.js";
import { op_console_size } from "ext:core/ops";
const {
  Uint32Array,
} = primordials;
const {
  isTerminal,
} = core;

const size = new Uint32Array(2);

function consoleSize() {
  op_console_size(size);
  return { columns: size[0], rows: size[1] };
}

function isatty(rid) {
  internals.warnOnDeprecatedApi(
    "system.isatty()",
    new Error().stack,
    "Use `system.stdin.isTerminal()`, `system.stdout.isTerminal()`, `system.stderr.isTerminal()` or `system.FsFile.isTerminal()` instead.",
  );
  return isTerminal(rid);
}

export { consoleSize, isatty };
