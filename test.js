
const express = require('express')
const app = express()
const port = 80

app.get('/', (req, res) => {
    res.send('Hello World!');
})

app.post('/', (req, res) => {
    res.sendStatus(200);
})

app.delete('/', (req, res) => {
    res.sendStatus(200);
})

app.listen(port, () => {
    console.log(`Example app listening at http://localhost:${port}`)
})