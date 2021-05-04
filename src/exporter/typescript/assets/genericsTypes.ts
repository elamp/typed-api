import { Request as ExpressRequest } from "express"

export type RequestMetadata = NodeJS.Dict<string>

export interface Request<QUERY> extends ExpressRequest {
  params: Partial<QUERY>
  query: Partial<QUERY>
  body: Partial<QUERY>
  session: any
}


export interface UseCase<PARAMS, RESULT> {
  execute: (sessionContext: any, requestMetadata: RequestMetadata, params: any) => Promise<Partial<RESULT>>,
}