async function getUser(user) {
  const response = await fetch(`http://127.0.0.1:3001/${user}`);
  const result = await response.text();
  return result;
}

for (const user of ['A', 'B', 'C', 'D']) {
  console.log(await getUser(user));
}
