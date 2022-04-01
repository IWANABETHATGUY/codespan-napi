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
export function emitError(
  fileName: string,
  sourceFile: string,
  labels: Array<LabelInfo>,
  errorMessage?: string | undefined | null,
): void
/** a wrapper of `codespan_reporting::diagnostic::Label` */
export class DiagnosticLabel {
  style: DiagnosticLabelStyle
  fileId: number
  info: LabelInfo
  static primary(fileId: number, info: LabelInfo): DiagnosticLabel
  static secondary(fileId: number, info: LabelInfo): DiagnosticLabel
}
export class FileMap {
  constructor()
  getFileId(fileName: string): number
  addFile(fileName: string, sourceFile: string): void
}
