<script lang="ts">
  import { onMount } from 'svelte'
  import { browser } from '$app/env'

  import { invoke } from '@tauri-apps/api/tauri'
  import { open, save } from '@tauri-apps/api/dialog'
  import { join } from '@tauri-apps/api/path'

  import { watch, watchImmediate } from 'tauri-plugin-fs-watch-api'

  import Fa from 'svelte-fa'
  import { LayerCake, Svg } from 'layercake'
  import InertiaPlot from '$lib/InertiaPlot.svelte'
  import { faExclamationTriangle, faSync } from '@fortawesome/free-solid-svg-icons'
  import Table from '$lib/Table.svelte'

  interface Generator {
    name: string
    bus: number
    pmin: number
    pmax: number
    basemva: number
    droop: number
    deadband: number
    h: number
  }
  let generator_data: Generator[] = []
  let generator_columns = [
    { title: 'Name', field: 'name' },
    { title: 'Bus', field: 'bus' },
    { title: 'Pmin', field: 'pmin' },
    { title: 'Pmax', field: 'pmax' },
    { title: 'Basemva', field: 'basemva' },
    { title: 'Droop', field: 'droop' },
    { title: 'Deadband', field: 'deadband' },
    { title: 'H', field: 'h' },
  ]

  let commitment_data: [] = []
  $: commitment_columns = generator_data
    .map((d) => d.name)
    .map((n) => {
      return {
        title: n,
        field: n.toLowerCase(),
      }
    })
  let stopWatching = null

  function _watchCallback(event) {
    const { type, payload } = event
    invoke('get_system_data', { data: payload })
      .then(async (m: Generator[]) => {
        console.log({ m })
        generator_data = m
      })
      .catch((error) => console.error(error))
  }

  async function _watch(filepath) {
    await _unwatch()
    stopWatching = await watch(filepath, { recursive: true }, _watchCallback).catch(_watchCallback)
  }

  async function _unwatch() {
    if (stopWatching) {
      await stopWatching().catch((error) => console.error(error))
      stopWatching = null
    }
  }

  async function handleClick(e) {
    const folder = await open({
      directory: true,
      multiple: false,
      title: 'Folder with system and commitment data',
    })
    const systemfilepath = join(folder[0], 'system-data.csv')
    invoke('get_system_data', { data: systemfilepath })
      .then(async (m: Generator[]) => {
        console.log({ m })
        generator_data = m
        if (stopWatching === null) {
          await _watch(systemfilepath).catch((error) => console.error(error))
        }
      })
      .catch((error) => console.error(error))

    const commitmentfilepath = join(folder[0], 'commitment-data.csv')
    invoke('get_commitment_data', { data: systemfilepath })
      .then(async (m: Generator[]) => {
        commitment_data = m
        if (stopWatching === null) {
          await _watch(systemfilepath).catch((error) => console.error(error))
        }
      })
      .catch((error) => console.error(error))
  }
</script>

<div class="flex mx-20 mt-6 space-x-2 justify-between content-center items-center">
  <h1 class="text-2xl py-2.5 font-medium leading-tight">Dashboard</h1>
  <div class="flex items-center space-x-6">
    <button
      on:click={async () => await _unwatch().catch((error) => console.error(error))}
      disabled={stopWatching === null}
    >
      <Fa icon={faSync} spin={stopWatching !== null} />
    </button>
    <button
      on:click={handleClick}
      type="button"
      class="inline-block px-6 py-2.5 bg-blue-600 text-white font-medium text-xs leading-tight uppercase rounded shadow-md hover:bg-blue-700 hover:shadow-lg focus:bg-blue-700 focus:shadow-lg focus:outline-none focus:ring-0 active:bg-blue-800 active:shadow-lg transition duration-150 ease-in-out"
      >Load Data</button
    >
  </div>
</div>

<div class="grid mx-20 my-4">
  {#if generator_data.length == 0}
    <div class="grid-flow-row w-full items-stretch">
      <div
        class="bg-yellow-100 rounded-lg py-5 px-6 mb-3 text-base text-yellow-700 inline-flex items-center w-full"
        role="alert"
      >
        <Fa class="w-4 h-4 mr-2 fill-current" icon={faExclamationTriangle} />
        No data loaded.
      </div>
    </div>
  {:else}
    <Table data={generator_data} columns={generator_columns} />
    <Table data={commitment_data} columns={commitment_columns} />
  {/if}
</div>
