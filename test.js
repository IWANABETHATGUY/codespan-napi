const { plus100 } = require('./index')
const file = `let a = 3;
let b = 4;

`
plus100("test.js", file, [{
  start: 10,
  end: 15,
  message: "something wrong"
}])