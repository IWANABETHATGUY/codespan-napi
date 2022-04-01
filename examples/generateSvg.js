// @ts-check
const { FileMap, Diagnostic, primaryDiagnosticLabel, secondaryDiagnosticLabel } = require('../index.js')
let map = new FileMap()

map.addFile(
  'test.fun',
  `
module FizzBuzz where

fizz₁ : Nat → String
fizz₁ num = case (mod num 5) (mod num 3) of
    0 0 => "FizzBuzz"
    0 _ => "Fizz"
    _ 0 => "Buzz"
    _ _ => num

fizz₂ : Nat → String
fizz₂ num =
    case (mod num 5) (mod num 3) of
        0 0 => "FizzBuzz"
        0 _ => "Fizz"
        _ 0 => "Buzz"
        _ _ => num
`.trim(),
)

const diagnostic = Diagnostic.error()

diagnostic.withMessage('`case` clauses have incompatible types')
diagnostic.withCode('E0308')
diagnostic.withLabels([
  primaryDiagnosticLabel(map.getFileId('test.fun'), {
    start: 328,
    end: 331,
    message: 'expected `String`, found `Nat`',
  }),
  secondaryDiagnosticLabel(map.getFileId('test.fun'), {
    start: 211,
    end: 331,
    message: '`case` clauses have incompatible types',
  }),
  secondaryDiagnosticLabel(map.getFileId('test.fun'), {
    start: 258,
    end: 268,
    message: 'this is found to be of type `String`',
  }),
  secondaryDiagnosticLabel(map.getFileId('test.fun'), {
    start: 284,
    end: 290,
    message: 'this is found to be of type `String`',
  }),
  secondaryDiagnosticLabel(map.getFileId('test.fun'), {
    start: 306,
    end: 312,
    message: 'this is found to be of type `String`',
  }),
  secondaryDiagnosticLabel(map.getFileId('test.fun'), {
    start: 186,
    end: 192,
    message: 'expected type `String` found here',
  }),
])
diagnostic.withNotes([
  `expected type \`String\`
  found type \`Nat\``,
])

diagnostic.emitSvg(map, "result.svg")
