const WebSocket = require("ws");

const PORT = 9001;
const wss = new WebSocket.Server({ port: PORT });

console.log(`Rafsan's Computer: JavaScript WebSocket server running on ws://127.0.0.1:${PORT}`);

wss.on("connection", (ws, req) => {
  const clientAddress = `${req.socket.remoteAddress}:${req.socket.remotePort}`;

  console.log(`Rafsan's Computer: client connected from ${clientAddress}`);

  ws.on("message", (data) => {
    const text = data.toString();

    console.log(`Rafsan's Computer: received from ${clientAddress}: ${text}`);

    wss.clients.forEach((client) => {
      if (client.readyState === WebSocket.OPEN) {
        client.send(text);
      }
    });
  });

  ws.on("close", () => {
    console.log(`Rafsan's Computer: client disconnected from ${clientAddress}`);
  });
});