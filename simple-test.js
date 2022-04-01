const { FileMap, DiagnosticLabel } = require('./index')


const m = new FileMap()
const a = DiagnosticLabel.primary(0, {});

m.addFile("test.js", "let a = 3")
m.addFile("test2.js", "let a = 3")