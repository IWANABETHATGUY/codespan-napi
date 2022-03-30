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
export function plus100(fileName: string, sourceFile: string, labels: Array<LabelMessage>): void
