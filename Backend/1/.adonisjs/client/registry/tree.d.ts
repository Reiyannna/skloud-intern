/* eslint-disable prettier/prettier */
import type { routes } from './index.ts'

export interface ApiDefinition {
  students: {
    index: typeof routes['students.index']
    show: typeof routes['students.show']
    store: typeof routes['students.store']
  }
}
