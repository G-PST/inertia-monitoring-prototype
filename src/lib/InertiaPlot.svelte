<!--
  @component
  Generates an SVG area shape using the `area` function from [d3-shape](https://github.com/d3/d3-shape).
 -->
<script lang="ts">
  import { Svg } from 'layercake'
  import { getContext } from 'svelte'
  const { data, width, height } = getContext('LayerCake')
  export let stroke = '#ab00d6'

  function linePlot(generator) {
    return (
      'M' +
      [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1]
        .map((d) => $width * d + ',' + $height * d)
        .join('L')
    )
  }
  console.log($data)
</script>

<Svg>
  <text>hello</text>
  {#each $data as line}
    <path class="path-line" d={linePlot(line)} {stroke} />
  {/each}
</Svg>
