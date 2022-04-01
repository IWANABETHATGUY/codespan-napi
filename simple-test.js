const { FileMap, Diagnostic, primaryDiagnosticLabel, secondaryDiagnosticLabel } = require('./index')

const m = new FileMap()
// const a = DiagnosticLabel.primary(0, {})

m.addFile(
  './simple-test.js',
  `let a = 3;
let b = 4;`,
)
m.addFile('test2.js', 'let a = 3')

let diagnostic = Diagnostic.warning()

diagnostic.withMessage('Something wrong')
diagnostic.withLabels([
  primaryDiagnosticLabel(m.getFileId('./simple-test.js'), { start: 8, end: 15, message: 'here should be a const' }),
  secondaryDiagnosticLabel(m.getFileId('test2.js'), { start: 0, end: 3, message: 'here should be a const' }),
])

diagnostic.emitSvg(m)
