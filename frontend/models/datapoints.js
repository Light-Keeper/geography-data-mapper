import { Toaster } from '@blueprintjs/core'
import { apiRequest } from '../lib/api'
import qsLib from 'qs'
import { useEffect, useState } from "react";

export async function getDatapoints({ datasourceId, filters, limit, order }) {
  let path = '/api/datapoints?' +
    qsLib.stringify({
      ...filters,
      dataset: datasourceId,
      limit,
      order_by: order
    })

  const page = await apiRequest('GET', path)
  return page.data
}

const ssr = typeof window === 'undefined'
const toast = !ssr && Toaster.create({ className: 'z-index-1000' })

export function useDatapoints({ datasourceId, filters, limit, order }) {
  const [{ data }, setData] = useState({ data: null })

  useEffect(() => {
    getDatapoints({ datasourceId, filters, limit, order })
      .then(data => setData({ data }))
      .catch(err => {
        console.error(err)
        toast.show({ message: `Failed to load datapoints for datasourceId=${datasourceId}`, intent: 'danger' })
      })
  }, [datasourceId, limit, filters, order])

  return { data }
}
