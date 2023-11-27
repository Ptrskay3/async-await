// Run with: `deno run bridge.js`.
function doSomethingThenCallback(f) {
  f();
}

async function asyncWrapper() {
  return await new Promise((resolve) => {
    doSomethingThenCallback(() => {
      console.log('in callback');
      resolve();
    });
  });
}

console.log('start executing');
await asyncWrapper();
console.log('asyncWrapper done..');
