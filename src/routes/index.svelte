<script lang="ts">
  import { SvelteToast } from '@zerodevx/svelte-toast'
  import { toast } from '@zerodevx/svelte-toast'

  import FileDrop from 'svelte-tauri-filedrop'
  import { onMount } from 'svelte'
  import { browser } from '$app/env'

  import { invoke } from '@tauri-apps/api/tauri'
  import { open, save } from '@tauri-apps/api/dialog'
  import { join } from '@tauri-apps/api/path'
  import path from 'path'

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
  function getCommitmentColumns(data: Generator[]) {
    let d = data
      .map((d) => d.name)
      .map((n) => {
        return {
          title: n,
          field: n,
        }
      })
    d.unshift({ title: 'Timestamp', field: 'timestamp' })
    return d
  }
  $: commitment_columns = getCommitmentColumns(generator_data)
  let stopWatchingSystemData = null
  let stopWatchingCommitmentData = null

  async function _unwatch() {
    console.log({ stopWatchingSystemData, stopWatchingCommitmentData })
    if (stopWatchingSystemData) {
      await stopWatchingSystemData().catch((error) => toastError(error))
      stopWatchingSystemData = null
    }
    if (stopWatchingCommitmentData) {
      await stopWatchingCommitmentData().catch((error) => toastError(error))
      stopWatchingCommitmentData = null
    }
  }

  async function handleClear(e) {
    stopWatchingSystemData = null
    stopWatchingCommitmentData = null
    commitment_data = []
    generator_data = []
  }

  async function handleLoadData(e) {
    const folder = await open({
      directory: true,
      multiple: false,
      title: 'Folder with system and commitment data',
    })
    const systemfilepath = await join(<string>folder, 'system-data.csv')
    const commitmentfilepath = await join(<string>folder, 'commitment-data.csv')
    const paths = [systemfilepath, commitmentfilepath]
    await handleFiles(paths).catch((err) =>
      toastError(`Unable to load system-data.csv and commitment-data.csv: ${err}.`),
    )
  }

  function toastError(m) {
    toast.push(m, {
      theme: {
        '--toastBackground': '#F56565',
        '--toastBarBackground': '#C53030',
      },
    })
  }
  async function handleFiles(paths) {
    if (paths.length != 2) {
      toastError('Two files required')
      return
    }
    if (paths.filter((p) => p.endsWith('system-data.csv')).length != 1) {
      toastError('system-data.csv required')
      return
    }
    if (paths.filter((p) => p.endsWith('commitment-data.csv')).length != 1) {
      toastError('commitment-data.csv')
      return
    }

    const systemfilepath = paths.filter((p) => p.endsWith('system-data.csv'))[0]
    generator_data = await invoke('get_system_data', { data: systemfilepath })
    if (stopWatchingSystemData === null) {
      stopWatchingSystemData = await watch(systemfilepath, { recursive: true }, (e) => {
        invoke('get_system_data', { data: e.payload })
          .then(async (m: Generator[]) => {
            generator_data = m
          })
          .catch((error) => console.error(error))
      })
    }

    const commitmentfilepath = paths.filter((p) => p.endsWith('commitment-data.csv'))[0]
    commitment_data = await invoke('get_commitment_data', {
      data: commitmentfilepath,
    })
    if (stopWatchingCommitmentData === null) {
      stopWatchingCommitmentData = await watch(commitmentfilepath, { recursive: true }, (e) => {
        invoke('get_commitment_data', { data: e.payload })
          .then(async (m: []) => {
            commitment_data = m
          })
          .catch((error) => console.error(error))
      })
    }
  }
</script>

<SvelteToast />

<div class="flex mx-20 mt-6 space-x-2 justify-between content-center items-center">
  <h1 class="text-2xl py-2.5 font-medium leading-tight">Dashboard</h1>
  <div class="flex items-center space-x-6">
    <button
      on:click={async () => await _unwatch().catch((err) => toastError(err))}
      disabled={stopWatchingSystemData === null && stopWatchingCommitmentData === null}
    >
      <Fa icon={faSync} spin={stopWatchingSystemData !== null} />
    </button>
    <button
      on:click={handleLoadData}
      type="button"
      class="inline-block px-6 py-2.5 bg-blue-600 text-white font-medium text-xs leading-tight uppercase rounded shadow-md hover:bg-blue-700 hover:shadow-lg focus:bg-blue-700 focus:shadow-lg focus:outline-none focus:ring-0 active:bg-blue-800 active:shadow-lg transition duration-150 ease-in-out"
      >Load Data</button
    >
    <button
      on:click={handleClear}
      type="button"
      class="inline-block px-6 py-2.5 bg-blue-600 text-white font-medium text-xs leading-tight uppercase rounded shadow-md hover:bg-blue-700 hover:shadow-lg focus:bg-blue-700 focus:shadow-lg focus:outline-none focus:ring-0 active:bg-blue-800 active:shadow-lg transition duration-150 ease-in-out"
      >Clear Data</button
    >
  </div>
</div>

<div class="mx-20 my-5">
  <FileDrop extensions={['csv']} {handleFiles} let:files>
    <div
      class="focus:text-gray-700 focus:bg-white focus:border-blue-600 focus:outline-none text-gray-700
          bg-gray-100 bg-clip-padding
          border border-solid border-gray-300 px-5 py-5"
      class:droppable={files.length > 0}
    >
      <h2>Drag and Drop CSV files here</h2>
    </div>
  </FileDrop>
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
    <Table data={commitment_data} columns={commitment_columns} id={false} />
  {/if}
</div>

<style>
  .droppable {
    background: #d6dff0;
  }
</style>
