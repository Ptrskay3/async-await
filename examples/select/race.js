// Run with: `deno run --allow-net race.js`.

async function fryEgg(egg) {
  const response = await fetch(`http://127.0.0.1:3001/${egg}`);
  const result = await response.text();
  return result;
}

const result = await Promise.race([
  fryEgg('A'),
  fryEgg('B'),
  fryEgg('C'),
  fryEgg('D'),
]);
console.log(result);
