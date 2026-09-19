import '@adonisjs/core/types/http'

type ParamValue = string | number | bigint | boolean

export type ScannedRoutes = {
  ALL: {
    'students.index': { paramsTuple?: []; params?: {} }
    'students.show': { paramsTuple: [ParamValue]; params: {'id': ParamValue} }
    'students.store': { paramsTuple?: []; params?: {} }
  }
  GET: {
    'students.index': { paramsTuple?: []; params?: {} }
    'students.show': { paramsTuple: [ParamValue]; params: {'id': ParamValue} }
  }
  HEAD: {
    'students.index': { paramsTuple?: []; params?: {} }
    'students.show': { paramsTuple: [ParamValue]; params: {'id': ParamValue} }
  }
  POST: {
    'students.store': { paramsTuple?: []; params?: {} }
  }
}
declare module '@adonisjs/core/types/http' {
  export interface RoutesList extends ScannedRoutes {}
}