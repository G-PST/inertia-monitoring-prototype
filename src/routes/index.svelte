<script>
  import { invoke } from '@tauri-apps/api/tauri'

  let files = {
    accepted: [],
    rejected: [],
  }
  async function handleClick(e) {
    let r = await invoke('my_custom_command', { invokeMessage: 'Hello!' })
    await invoke('write_report')
    console.log(r)
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
