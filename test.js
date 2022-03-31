const { emitDiagnostic } = require('./index')
const file = `let a = 3;
let b = 4;

`
emitDiagnostic("test.js", file, [{
  start: 10,
  end: 15,
  message: "something wrong"
}], "that is a wrong")