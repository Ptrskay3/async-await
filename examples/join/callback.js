// Run with: `deno run --allow-net callback.js`
function fryEgg(name) {
  return fetch(`http://127.0.0.1:3001/${name}`).then((resp) => resp.text());
}

const EGGS = ['A', 'B', 'C', 'D'];

function join_all(callback) {
  let completedTasks = 0;

  function checkCompletion() {
    completedTasks++;
    if (completedTasks === EGGS.length) {
      callback();
    }
  }

  for (const egg of EGGS) {
    fryEgg(egg).then((res) => {
      console.log(res);
      checkCompletion();
    });
  }
}

join_all(() => {});
