function getUser(user) {
  return fetch(`http://127.0.0.1:3001/${user}`).then((resp) => resp.text());
}

const USERS = ['A', 'B', 'C', 'D'];

function join_all(callback) {
  let completedTasks = 0;

  function checkCompletion() {
    completedTasks++;
    if (completedTasks === USERS.length) {
      callback();
    }
  }

  for (const user of USERS) {
    getUser(user).then((res) => {
      console.log(res);
      checkCompletion();
    });
  }
}

join_all(() => {});
