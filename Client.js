const net = require("net");
const readline = require("readline");

const client = new net.Socket();
const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});

client.connect(1234, "127.0.0.1", () => {
  console.log("Bienvenue sur le client de chat multicanal!");
  rl.question("Entrez votre nom d'utilisateur : ", (username) => {
    client.write(username);

    rl.setPrompt(``);
    rl.prompt();
  });
});

client.on("data", (data) => {
  console.log("\n" + data);
  process.stdout.write(data);
  rl.prompt();
});

rl.on("line", (line) => {
  if (line === "") {
    client.end();
  } else {
    client.write(`${line}\n`);
  }
});

client.on("close", () => {
  console.log("La connexion a été fermée.");
  process.exit(0);
});
