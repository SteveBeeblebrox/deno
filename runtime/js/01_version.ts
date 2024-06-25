// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

import { primordials } from "ext:core/mod.js";
const {
  ObjectFreeze,
} = primordials;

interface Version {
  sjs: string;
  v8: string;
  typescript: string;
}

const version: Version = {
  sjs: "",
  v8: "",
  typescript: "",
};

function setVersions(
  denoVersion,
  v8Version,
  tsVersion,
) {
  version.sjs = denoVersion;
  version.v8 = v8Version;
  version.typescript = tsVersion;

  ObjectFreeze(version);
}

export { setVersions, version };
