// Run with: `deno run --allow-net blocking.js`.
async function fryEgg(egg) {
  const response = await fetch(`http://127.0.0.1:3001/blocking/${egg}`);
  const result = await response.text();
  return result;
}

const eggs = ['A', 'B', 'C', 'D'];

const response = await Promise.all(eggs.map((egg) => fryEgg(egg)));
console.log(response);
