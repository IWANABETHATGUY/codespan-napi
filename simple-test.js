const fs = require('fs')
const path = require('path')
const { FileMap, Diagnostic, primaryDiagnosticLabel, secondaryDiagnosticLabel, positionToOffset } = require('./index')

const index = fs.readFileSync('dist/examples/index.js').toString()
const result = fs.readFileSync('dist/examples/result.js').toString()
const m = new FileMap()
// const a = DiagnosticLabel.primary(0, {})

m.addFile('dist/examples/index.js', index)
m.addFile('dist/examples/result.js', result)

let diagnostic = Diagnostic.error()
diagnostic.withMessage('Something wrong')
diagnostic.withLabels([
  primaryDiagnosticLabel(m.getFileId('dist/examples/result.js'), {
    start: positionToOffset(result, 1, 6),
    end: positionToOffset(result, 1, 7),
    message: "Variable `a` can't be redefined",
  }),
  secondaryDiagnosticLabel(m.getFileId('dist/examples/result.js'), {
    start: positionToOffset(result, 0, 4),
    end: positionToOffset(result, 0, 5),
    message: 'Variable `a` first defined here',
  }),
])

diagnostic.emitStd(m)
