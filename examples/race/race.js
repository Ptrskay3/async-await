async function main() {
  const result = await Promise.race([
    new Promise((resolve) => {
      setTimeout(() => resolve('apple'), 1000);
    }),
    new Promise((_, reject) => {
      setTimeout(() => reject(new Error('Request timed out')), 5000);
    }),
  ]);
  console.log(result);
}

main();
