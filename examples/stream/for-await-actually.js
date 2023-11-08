async function fryEgg(egg) {
  const response = await fetch(`http://127.0.0.1:3001/${egg}`);
  const result = await response.text();
  return result;
}

const eggs = ['A', 'B', 'C', 'D'];

function* fryEggGenerator() {
  for (const egg of eggs) {
    yield fryEgg(egg);
  }
}

for await (const egg of fryEggGenerator()) {
  console.log(egg);
}
