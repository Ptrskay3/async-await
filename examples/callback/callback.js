const http = require('http');

for (const user of ['A', 'B', 'C', 'D']) {
  http
    .get(`http://localhost:3001/${user}`, (res) => {
      let data = [];
      res.on('data', (chunk) => {
        data.push(chunk);
      });

      res.on('end', () => {
        console.log(Buffer.concat(data).toString());
      });
    })
    .on('error', (err) => {
      console.log('Error: ', err.message);
    });
}