// @ts-check
const {
  FileMap,
  Diagnostic,
  primaryDiagnosticLabel,
  secondaryDiagnosticLabel,
  createLabelInfo,
} = require('../index.js')
let map = new FileMap()

map.addFile(
  'test.ts',
  `
export function hello(a: string) {

}
`.trim(),
)

map.addFile(
  'test2.ts',
  `
import {hello} from 'test.ts'
hello(2222)
`.trim(),
)

const diagnostic = Diagnostic.error()

diagnostic.withMessage("Argument of type 'number' is not assignable to parameter of type 'string'")
diagnostic.withCode('2345')
diagnostic.withLabels([
  primaryDiagnosticLabel(map.getFileId('test.ts'), createLabelInfo(22, 31, 'expected first param has type `string`')),
  primaryDiagnosticLabel(map.getFileId('test2.ts'), createLabelInfo(36,40, 'the first argument type is `number`')),
])
diagnostic.withNotes([
  `expected type \`string\`
  found type \`number\``,
])

// diagnostic.emitStd(map)
diagnostic.emitSvg(map, "multiple_file.svg")
