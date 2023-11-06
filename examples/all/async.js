async function fryEgg(user) {
  const response = await fetch(`http://127.0.0.1:3001/${user}`);
  const result = await response.text();
  return result;
}

const eggs = ['A', 'B', 'C', 'D'];

const response = await Promise.all(eggs.map((user) => fryEgg(user)));
console.log(response);
