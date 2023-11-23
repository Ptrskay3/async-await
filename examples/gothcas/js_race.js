let sharedVariable = 0;

async function incrementAsync() {
  let temp = sharedVariable; // read a shared variable
  await new Promise((resolve) => setTimeout(resolve, 100)); // wait for 100 milliseconds
  temp++; // increment the local copy
  sharedVariable = temp; // write back to the shared variable
}

await Promise.all([incrementAsync(), incrementAsync()]);

console.log('Final value:', sharedVariable);
