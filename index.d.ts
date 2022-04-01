/* eslint-disable */

export class ExternalObject<T> {
  readonly '': {
    readonly '': unique symbol
    [K: symbol]: T
  }
}
export interface LabelMessage {
  message: string
  start: number
  end: number
}
export function emitError(
  fileName: string,
  sourceFile: string,
  labels: Array<LabelMessage>,
  errorMessage?: string | undefined | null,
): void
export class FileMap {
  constructor()
  getFileId(fileName: string): number
  addFile(fileName: string, sourceFile: string): void
}
