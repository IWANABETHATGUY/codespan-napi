/* eslint-disable */

export class ExternalObject<T> {
  readonly '': {
    readonly '': unique symbol
    [K: symbol]: T
  }
}
export interface LabelInfo {
  message: string
  start: number
  end: number
}
export const enum DiagnosticLabelStyle {
  Primary = 0,
  Secondary = 1,
}
/** a wrapper of `codespan_reporting::diagnostic::Label` */
export interface DiagnosticLabel {
  style: DiagnosticLabelStyle
  fileId: number
  info: LabelInfo
}
export function primaryDiagnosticLabel(fileId: number, info: LabelInfo): DiagnosticLabel
export function secondaryDiagnosticLabel(fileId: number, info: LabelInfo): DiagnosticLabel
export const enum Severity {
  Bug = 0,
  Error = 1,
  Warning = 2,
  Note = 3,
  Help = 4,
}
export function emitError(
  fileName: string,
  sourceFile: string,
  labels: Array<LabelInfo>,
  errorMessage?: string | undefined | null,
): void
export class FileMap {
  constructor()
  getFileId(fileName: string): number
  addFile(fileName: string, sourceFile: string): void
}
export class Diagnostic {
  static error(): Diagnostic
  static bug(): Diagnostic
  static warning(): Diagnostic
  static help(): Diagnostic
  static note(): Diagnostic
  withMessage(message: string): void
  withCode(code: string): void
  withLabels(labels: Array<DiagnosticLabel>): void
  withNotes(notes: Array<string>): void
  emitStd(fileMap: FileMap): void
  emitSvg(fileMap: FileMap, path: string): void
}
