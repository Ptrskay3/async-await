async function getUser(user) {
  const response = await fetch(`http://127.0.0.1:3001/${user}`);
  const result = await response.text();
  console.log(result);
}

const users = ['A', 'B', 'C', 'D'];

await Promise.all(users.map((user) => getUser(user)));
