/* eslint-disable prettier/prettier */
import type { AdonisEndpoint } from '@tuyau/core/types'
import type { Registry } from './schema.d.ts'
import type { ApiDefinition } from './tree.d.ts'

const placeholder: any = {}

const routes = {
  'students.index': {
    methods: ["GET","HEAD"],
    pattern: '/students',
    tokens: [{"old":"/students","type":0,"val":"students","end":""}],
    types: placeholder as Registry['students.index']['types'],
  },
  'students.show': {
    methods: ["GET","HEAD"],
    pattern: '/students/:id',
    tokens: [{"old":"/students/:id","type":0,"val":"students","end":""},{"old":"/students/:id","type":1,"val":"id","end":""}],
    types: placeholder as Registry['students.show']['types'],
  },
  'students.store': {
    methods: ["POST"],
    pattern: '/students',
    tokens: [{"old":"/students","type":0,"val":"students","end":""}],
    types: placeholder as Registry['students.store']['types'],
  },
} as const satisfies Record<string, AdonisEndpoint>

export { routes }

export const registry = {
  routes,
  $tree: {} as ApiDefinition,
}

declare module '@tuyau/core/types' {
  export interface UserRegistry {
    routes: typeof routes
    $tree: ApiDefinition
  }
}
