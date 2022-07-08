<!--
  @component
  Generates an SVG area shape using the `area` function from [d3-shape](https://github.com/d3/d3-shape).
 -->
<script lang="ts">
  import * as d3 from "d3";
  import { LayerCake, ScaledSvg, Svg, Html } from "layercake";
  import { getContext } from "svelte";
  const { data, width, height } = getContext("LayerCake");
  export let stroke = "#ab00d6";
  export let inertiaFloorStroke = "#ff0000";

  $: last_slice = $data.last_slice;

  $: timeScale = d3
    .scaleTime()
    .domain(
      d3.extent($data.commitment_data.slice(-1 * last_slice), (d) => {
        let t = new Date(d.timestamp);
        t.setTime(t.getTime() + t.getTimezoneOffset() * 60 * 1000);
        return t;
      }),
    )
    .range([0, $width]);
  $: inertiaScale = d3
    .scaleLinear()
    .domain([0, d3.extent($data.commitment_data, (d) => inertiaRow(d))[1] * 1.2])
    .range([$height, 0]);

  function inertiaRow(row) {
    let h = 0;
    for (const gen of $data.generator_data) {
      h += parseInt(row[gen.name] ? row[gen.name] : "0") * gen.h * gen.mva;
    }
    return h;
  }

  let xaxis = null;
  let yaxis = null;
  $: d3.select(xaxis).call(d3.axisBottom(timeScale).tickFormat(d3.timeFormat("%H:%M:%S")));
  $: d3.select(yaxis).call(d3.axisLeft(inertiaScale));

  function getLinePlot(data, width) {
    const lineplot = [];
    for (const row of data.commitment_data.slice(-1 * last_slice)) {
      let h = 0;
      for (const gen of data.generator_data) {
        h += parseInt(row[gen.name] ? row[gen.name] : "0") * gen.h * gen.mva;
      }
      let d = new Date(row.timestamp);
      d.setTime(d.getTime() + d.getTimezoneOffset() * 60 * 1000);
      lineplot.push({ timestamp: timeScale(d), value: inertiaScale(h) });
    }
    return lineplot;
  }

  function getInertiaFloorLinePlot(data, width) {
    const lineplot = [];
    for (const row of data.commitment_data.slice(-1 * last_slice)) {
      let d = new Date(row.timestamp);
      d.setTime(d.getTime() + d.getTimezoneOffset() * 60 * 1000);
      lineplot.push({ timestamp: timeScale(d), value: inertiaScale(data.inertia_floor) });
    }
    return lineplot;
  }
  $: path =
    "M" +
    getLinePlot($data, $width)
      .map((d) => {
        return d.timestamp + "," + d.value;
      })
      .join("L");
  $: inertiaFloorPath =
    "M" +
    getInertiaFloorLinePlot($data, $width)
      .map((d) => {
        return d.timestamp + "," + d.value;
      })
      .join("L");
</script>

<Svg>
  <g transform="translate(100 100)">
    <g class="xaxis" bind:this={xaxis} transform="translate(0  {$height})">
      <text
        opacity="1"
        fill="currentColor"
        y="9"
        dy="0.71em"
        font-size="12px"
        text-anchor="middle"
        transform="translate({$width / 2} 25)"
      >
        Time (HH:MM:SS)
      </text>
    </g>
    <g class="yaxis" bind:this={yaxis}>
      <text
        opacity="1"
        fill="currentColor"
        y="9"
        dy="0.71em"
        font-size="12px"
        text-anchor="middle"
        transform="rotate(270) translate({-$height / 2} {-$width / 25})"
      >
        Inertia (MWs)
      </text>
    </g>
    <path class="path-line" fill="none" d={path} {stroke} />
    <path class="path-line" fill="none" d={inertiaFloorPath} stroke={inertiaFloorStroke} />
  </g>
</Svg>
