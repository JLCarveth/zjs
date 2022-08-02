console.log("Hello, zjs!")
console.error("Uh oh...");

const path = "./log.txt";

try {
    const contents = await zjs.readFile(patn);
    console.log("Read from a file", contents);
} catch (err) {
    console.error("Unable to read file", path, err);
}

await zjs.writeFile(path, "I can write to a file.");
const contents = await zjs.readFile(path);
console.log("Read from a file", path, "contents:", contents);
console.log("Removing file", path);
zjs.removeFile(path);
console.log("Removed file")

// Try to send an HTTP request
let response = await http.get("https://www.uuidtools.com/api/generate/v4");
console.log(response);