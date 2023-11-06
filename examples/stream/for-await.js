async function fryEgg(name) {
  const response = await fetch(`http://127.0.0.1:3001/${name}`);
  const result = await response.text();
  return result;
}

const eggs = ['A', 'B', 'C', 'D'];
const promises = eggs.map((name) => fryEgg(name));

for await (const egg of promises) {
  console.log(egg);
}
