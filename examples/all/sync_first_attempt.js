function fryEgg(egg) {
  return fetch(`http://127.0.0.1:3001/${egg}`).then((resp) => resp.text());
}

for (const egg of ['A', 'B', 'C', 'D']) {
  fryEgg(egg).then((res) => console.log(res));
}
