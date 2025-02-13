<template>
  <div
    id="map-container"
    class="bg-slate-100 rounded-xl relative"
    @mousemove="onMouseMove"
  >
    <svg
      ref="canvas"
      id="map"
      class="text-primary stroke-white fill-slate-300 size-full"
    />

    <div
      ref="tooltip"
      :class="[
        'border invisible',
        'rounded-lg shadow-xl w-max absolute left-1/2 top-0 bg-white',
        '-translate-x-1/2',
      ]"
    >
      <slot name="tooltip" :data="tooltipData" />
    </div>
  </div>
</template>

<script setup lang="ts" generic="T">
import {
  select,
  geoMercator,
  geoPath,
  json as jsonLoader,
  selectAll,
  geoAlbersUsa,
} from "d3";
import { feature } from "topojson-client";
import { Debouncer } from "~/_common/utils";

interface IProps {
  shapeIdField: string;
  layerName: string;
  projection?: "us" | string;

  /**
   * The url to the topojson file.
   *
   * Should only be passed if `geojson` is not provided
   */
  json?: string;

  /**
   * The parsed geojson file.
   *
   * Should be passed if `json` is not provided
   */
  geojson?: any | null;

  /**
   * The map's area in sq. km
   */
  area: number;

  /**
   * The scale to which the map is drawn.
   *
   * The map's `area` is divided by the `scale` to determine how big to draw the map
   */
  scale?: number;

  /**
   * The map's center in [longitude, latitude]
   */
  center: [number, number];

  getTooltipData?: (regionId: string) => T;
  regionClass?: (data: any) => string | null;
  regionStyle?: (data: any) => string | null;
}
const props = withDefaults(defineProps<IProps>(), {
  projection: "mercator",
  scale: 1,
});
const emit = defineEmits<{
  regionClick: [attribute: string];
}>();

const tooltip = ref<HTMLDivElement | null>(null);
const tooltipData = ref<T | null>(null);

const canvas = ref<SVGElement | null>(null);
const canvasSize = ref<{
  x: number;
  y: number;
  width: number;
  height: number;
} | null>(null);

watch(canvas, (c, prevC) => {
  if (c === null) return;
  if (prevC !== null) return;

  const { width, height, x, top: y } = c.getBoundingClientRect();
  canvasSize.value = { x, y, width, height };

  loadData();
});

useResizeObserver(canvas, (entries) => {
  console.log("Resize");
  const entry = entries[0];
  const { width, height, x, top: y } = entry.contentRect;
  canvasSize.value = { x, y, width, height };
});

const geojson = ref<any | null>(null);
watch(() => props.geojson, loadData);
watch(() => props.area, drawMap);
watch(() => props.center, drawMap);

const debouncer = new Debouncer(100);
watch(canvasSize, () => debouncer.run(drawMap));
watch(geojson, drawMap);

async function loadData() {
  const propJson = props.json;
  const gJson = props.geojson;

  if (!propJson && !gJson) {
    console.error('Either "json" or "geojson" need to be provided');
    return;
  }

  if (gJson) {
    geojson.value = gJson;
    return;
  }

  const data = await jsonLoader<any>(propJson!);
  if (!data) return;

  geojson.value = feature(data, data.objects[props.layerName]) as any;
}

let isDrawing = false;
function drawMap() {
  const gJson = geojson.value;
  if (!gJson || isDrawing) return;

  const cSize = canvasSize.value;
  if (cSize === null) return;

  isDrawing = true;
  // Clear all existing paths
  selectAll("svg#map > *").remove();

  const svg = select("svg#map");
  const { width, height } = cSize;
  const center = props.center;

  /* ------------------ Scale Calculation Magic ----------------- */
  const landMass = props.area / props.scale;

  const baseScale = 350; // Scale used to draw Africa
  const baseLandMass = 30.365e6; // Africa's land mass
  const k = baseLandMass * baseScale ** 2; // Constant of proportionality (from landMass (A) = k / (scale(S)^2))
  const scale = Math.sqrt(k / landMass);
  /* -------------- End of Scale Calculation Magic -------------- */

  const projection =
    props.projection === "us"
      ? geoAlbersUsa()
      : geoMercator().center(center as [number, number]);
  projection.scale(scale).translate([width / 2, height / 2]);
  if (props.projection === "us") {
    projection.scale(scale * 1.3);
  }

  const pathGenerator = geoPath().projection(projection);
  const paths = svg
    .append("g")
    .selectAll("path")
    .data(gJson.features)
    .enter()
    .append("path")
    .attr("opacity", 0.8)
    .attr("d", (d: any) => pathGenerator(d))
    .attr("data-name", (d: any) => d.properties[props.shapeIdField])
    .attr("class", (d: any) => {
      let klass: string | null = "";
      const regionClass = props.regionClass;
      if (regionClass) klass = regionClass(d);

      let klasses = "region";
      if (klass) klasses += ` ${klass}`;
      return klasses;
    });

  const regionStyle = props.regionStyle;
  if (regionStyle) {
    paths.attr("style", regionStyle);
  }

  paths
    .on("mouseover", onPathMouseOver)
    .on("mouseleave", onPathMouseLeave)
    .on("click", onClick);

  isDrawing = false;
}

function onMouseMove({ target, offsetX, offsetY }: MouseEvent) {
  const el = target as SVGPathElement | SVGElement;

  const cSize = canvasSize.value;
  const t = tooltip.value;
  if (!t || !cSize) return;

  if (el.nodeName === "svg") return;

  tooltipData.value = props.getTooltipData?.(el.getAttribute("data-name")!);
  let x = offsetX;
  let y = offsetY + 20;

  t.style.left = `${x}px`;
  t.style.top = `${y}px`;
}

function onPathMouseOver({ target }: MouseEvent) {
  const el = target as SVGPathElement;
  select(el).style("opacity", 1);

  const t = tooltip.value;
  if (!t) return;

  t.style.visibility = "visible";
  return;
}

function onPathMouseLeave({ target }: MouseEvent) {
  const el = target as SVGElement | SVGPathElement;
  select(target as SVGPathElement).style("opacity", 0.8);

  const t = tooltip.value;
  if (!t) return;

  t.style.visibility = "hidden";
  return;
}

function onClick({ target }: MouseEvent) {
  const el = target as SVGPathElement;
  const attribute = el.getAttribute("data-name") as string;

  if (attribute) emit("regionClick", attribute);
}
</script>
