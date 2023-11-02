async function getUser(name) {
  const response = await fetch(`http://127.0.0.1:3001/${name}`);
  const result = await response.text();
  return result;
}

const users = ['A', 'B', 'C', 'D'];
const promises = users.map((name) => getUser(name));

function* user_generator() {
  for (const user of users) {
    yield getUser(user);
  }
}

for await (const user of user_generator()) {
  console.log(user);
}
