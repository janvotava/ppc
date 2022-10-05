<script lang="ts">
  import { DateTime } from "luxon"
  import type { PageData } from "./$types"

  export let data: PageData

  function formatDateTime(datetime: string) {
    const luxonDate = DateTime.fromISO(datetime)

    return luxonDate.toLocaleString(DateTime.DATETIME_SHORT_WITH_SECONDS)
  }
</script>

<table class="table-auto w-full">
  <thead class="text-md font-semibold uppercase text-gray-400 bg-gray-50">
    <tr class="font-semibold">
      <th class="p-2 whitespace-nowrap text-left">
        Clicked on
      </th>
      <th class="p-2 whitespace-nowrap text-left">
        IP
      </th>
      <th class="p-2 text-left">
        Campaign
      </th>
      <th class="p-2 whitespace-nowrap text-left">
        Keyword
      </th>
    </tr>
  </thead>

  <tbody class="text-md divide-y divide-gray-100">
    {#each data.clicks as click}
      <tr>
        <td class="p-2 whitespace-nowrap">
          {formatDateTime(click.created_at)}
        </td>

        <td class="p-2 whitespace-nowrap flex flex-col">
          <div>
            {#if click.country_iso_code}
              <strong>{click.country_iso_code}</strong>
              &nbsp;
            {/if}

            {click.ip}
          </div>

          {#if click.asn_organization}
            <div class="text-sm text-gray-500">
              {click.asn_organization}

              {#if click.asn_number}
                ({click.asn_number})
              {/if}
            </div>
          {/if}
        </td>

        <td class="p-2">
          {#if click.campaignid}
            {click.campaignid}
          {/if}
        </td>

        <td class="p-2 whitespace-nowrap">
          {click.keyword}
        </td>
      </tr>
    {/each}
  </tbody>
</table>