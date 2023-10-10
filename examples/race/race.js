async function getUser(user) {
  const response = await fetch(`http://127.0.0.1:3001/${user}`);
  const result = await response.text();
  return result;
}

async function main() {
  const result = await Promise.race([
    getUser('A'),
    new Promise((_, reject) => {
      setTimeout(() => reject(new Error('Request timed out')), 1100);
    }),
  ]);
  console.log(result);
}

main();
