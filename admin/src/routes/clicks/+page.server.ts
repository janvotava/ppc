import type { PageServerLoad } from "./$types"
import Surreal, { type Result } from "surrealdb.js"
import { error } from "@sveltejs/kit"

interface Click {
  url: string
  ad?: string
  adgroup?: string
  campaignid?: string
  created_at: string
  device?: string
  devicemodel?: string
  gclid?: string
  id?: string
  ip?: string
  keyword?: string
  network?: string
  placement?: string

  country_iso_code?: string
  asn_number?: number
  asn_organization?: string
}

export const load: PageServerLoad = async () => {
  const db = new Surreal("http://db:8000/rpc")

  await db.signin({
    user: "root",
    pass: "root",
  })

  await db.use("ppc", "ppc")

  const response = await db.query<[Result<Click[]>]>("SELECT * FROM click ORDER BY created_at DESC LIMIT 200")

  if (response[0].error) {
    throw error(500, response[0].error.message);
  }

  return {
    clicks: response[0].result,
  }
}
