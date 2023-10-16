function getUser(user) {
  return fetch(`http://127.0.0.1:3001/${user}`).then((resp) => resp.text());
}

for (const user of ['A', 'B', 'C', 'D']) {
  getUser(user).then((res) => console.log(res));
}
