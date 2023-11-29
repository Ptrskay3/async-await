// Run with: `deno run --allow-net sync.js`
async function fryEgg(egg) {
  const response = await fetch(`http://127.0.0.1:3001/${egg}`);
  const result = await response.text();
  return result;
}

for (const egg of ['A', 'B', 'C', 'D']) {
  console.log(await fryEgg(egg));
}
