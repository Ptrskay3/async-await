async function getUser(user) {
  const response = await fetch(`http://127.0.0.1:3001/${user}`);
  const result = await response.text();
  return result;
}

const users = ['A', 'B', 'C', 'D'];

const response = await Promise.all(users.map((user) => getUser(user)));
console.log(response);
