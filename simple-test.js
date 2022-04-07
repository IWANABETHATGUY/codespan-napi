const fs = require('fs')
const path = require('path')
const { FileMap, Diagnostic, primaryDiagnosticLabel, secondaryDiagnosticLabel } = require('./index')

const index = fs.readFileSync('dist/examples/index.js').toString()
const result = fs.readFileSync('dist/examples/result.js').toString()
const m = new FileMap()
// const a = DiagnosticLabel.primary(0, {})

m.addFile('dist/examples/index.js', index)
m.addFile('dist/examples/result.js', result)

let diagnostic = Diagnostic.warning()

diagnostic.withMessage('Something wrong')
diagnostic.withLabels([
  primaryDiagnosticLabel(m.getFileId('dist/examples/result.js'), { start: 17, end: 18, message: 'Variable a can\'t be redefined' }),
  secondaryDiagnosticLabel(m.getFileId('dist/examples/result.js'), { start: 4, end: 5, message: 'Variable a first defined here' }),
])

diagnostic.emitStd(m)
