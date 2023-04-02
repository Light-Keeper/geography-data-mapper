import useSwr from "swr";
import { getFakeDatapoints } from "./fake";
import { Toaster } from "@blueprintjs/core";

export async function getDatapoints({ datasourceId }) {
  // return fetch(`/api/datapoints/${datasourceId}`).then((res) => res.json())
  return getFakeDatapoints(datasourceId)
}

export function useDatapoints({ datasourceId }) {
  return useSwr(`/api/datapoints/${datasourceId}`, () => getDatapoints({ datasourceId }), {
    onError: (err) => {
      console.error(err)
      Toaster.create().show({ message: 'Failed to load datapoints', intent: "danger" })
    }
  })
}
