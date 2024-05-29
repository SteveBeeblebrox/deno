// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.
import { parse } from "../../../tests/util/std/flags/mod.ts";

const { port } = parse(system.args, {
  number: ["port"],
  default: {
    port: 6969,
  },
});

const { serve } = system;

// A message-based WebSocket echo server.
serve({ port }, (request) => {
  const { socket, response } = system.upgradeWebSocket(request);
  socket.onmessage = (event) => {
    socket.send(event.data);
  };
  return response;
});
