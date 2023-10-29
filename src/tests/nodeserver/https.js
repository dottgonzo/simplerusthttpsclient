const fs = require("fs");
const https = require("https");

const privateKey = fs.readFileSync("server_key.pem", "utf8");
const certificate = fs.readFileSync("server_cert.pem", "utf8");
const credentials = { key: privateKey, cert: certificate };

const httpsServer = https.createServer(credentials, (req, res) => {
  const headers = req.headers;
  console.info("headers:", headers);
  res.writeHead(200, { "Content-Type": "application/json" });
  res.end(JSON.stringify({ ok: true }));
});

httpsServer.listen(3000, () => {
  console.log("Server HTTPS in ascolto sulla porta 3000");
});
