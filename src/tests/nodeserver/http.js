const http = require("http");

// Carica i certificati

const httpServer = http.createServer((req, res) => {
  const headers = req.headers;
  console.info("headers:", headers);
  res.writeHead(200, { "Content-Type": "application/json" });
  res.end(JSON.stringify({ ok: true }));
});

httpServer.listen(3000, () => {
  console.log("Server HTTPS in ascolto sulla porta 3000");
});
