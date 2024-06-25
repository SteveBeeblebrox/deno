// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.
import { notImplemented } from "ext:deno_node/_utils.ts";

export default class Dirent {
  constructor(private entry: system.DirEntry) {}

  isBlockDevice(): boolean {
    notImplemented("system does not yet support identification of block devices");
    return false;
  }

  isCharacterDevice(): boolean {
    notImplemented(
      "system does not yet support identification of character devices",
    );
    return false;
  }

  isDirectory(): boolean {
    return this.entry.isDirectory;
  }

  isFIFO(): boolean {
    notImplemented(
      "system does not yet support identification of FIFO named pipes",
    );
    return false;
  }

  isFile(): boolean {
    return this.entry.isFile;
  }

  isSocket(): boolean {
    notImplemented("system does not yet support identification of sockets");
    return false;
  }

  isSymbolicLink(): boolean {
    return this.entry.isSymlink;
  }

  get name(): string | null {
    return this.entry.name;
  }

  get parentPath(): string {
    return this.entry.parentPath;
  }

  /** @deprecated */
  get path(): string {
    return this.parentPath;
  }
}
