async function fryEgg(user) {
  const response = await fetch(`http://127.0.0.1:3001/${user}`);
  const result = await response.text();
  return result;
}

const result = await Promise.race([
  fryEgg('A'),
  fryEgg('B'),
  fryEgg('C'),
  fryEgg('D'),
  // new Promise((_, reject) => {
  //   setTimeout(() => reject(new Error('Request timed out')), 1100);
  // }),
]);
console.log(result);
