const { FileMap } = require('./index')


const m = new FileMap()

m.addFile("test.js", "let a = 3")
m.addFile("test2.js", "let a = 3")
console.log(m.getFileId("test.js"))
console.log(m.getFileId("test2.js"))