async function getUser(user) {
  let result = await fetch(`http://127.0.0.1:3001/${user}`).then((data) => data.text());
  return result;
}

const users = ['A', 'B', 'C', 'D'];

const promises = users.map((user) => getUser(user));

const results = [];
for await (const avatar of promises) {
  results.push(avatar);
}
console.log(results);
