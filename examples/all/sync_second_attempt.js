// Run with: `deno run --allow-net sync_second_attempt.js`.
async function fryEgg(name) {
  const response = await fetch(`http://127.0.0.1:3001/${name}`);
  const result = await response.text();
  return result;
}

for (const egg of ['A', 'B', 'C', 'D']) {
  console.log(await fryEgg(egg));
}
