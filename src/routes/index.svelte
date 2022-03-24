<script lang="ts">
  import { onMount } from 'svelte'
  import { browser } from '$app/env'

  import { invoke } from '@tauri-apps/api/tauri'
  import { open, save } from '@tauri-apps/api/dialog'

  import { LayerCake, Svg } from 'layercake'
  import InertiaPlot from '$lib/InertiaPlot.svelte'

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
  let data: Generator[] = []
  function get_system_data(filepath) {
    invoke('get_system_data', { data: filepath })
      .then((m: Generator[]) => (data = m))
      .catch((error) => console.error(error))
  }
  let files = {
    accepted: [],
    rejected: [],
  }
  async function handleClick(e) {
    let filepath = await open()
    get_system_data(filepath)
  }
</script>

<div class="grid mx-20 mt-6">
  <h1 class="text-2xl font-medium leading-tight mt-0 mb-2">Dashboard</h1>
</div>

<div class="flex space-x-2 justify-center">
  <button
    on:click={handleClick}
    type="button"
    class="inline-block px-6 py-2.5 bg-blue-600 text-white font-medium text-xs leading-tight uppercase rounded shadow-md hover:bg-blue-700 hover:shadow-lg focus:bg-blue-700 focus:shadow-lg focus:outline-none focus:ring-0 active:bg-blue-800 active:shadow-lg transition duration-150 ease-in-out"
    >Button</button
  >
</div>

<div class="grid mx-20 my-4 h-5/6">
  <LayerCake padding={{ right: 10, bottom: 20, left: 25 }} yDomain={[0, null]} {data}>
    <InertiaPlot />
  </LayerCake>
</div>
