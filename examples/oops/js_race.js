let sharedVariable = 0;

async function incrementAsync() {
  let temp = sharedVariable; // read a shared variable
  await new Promise((resolve) => setTimeout(resolve, 100)); // wait for 100 milliseconds
  temp++; // Increment local copy
  sharedVariable = temp; // Write back to a shared variable
}

await Promise.all([incrementAsync(), incrementAsync()]);

console.log('Final value:', sharedVariable);
