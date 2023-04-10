// Functionality related to coloring the data-points on a map

import { Colors } from "@blueprintjs/core";

const colorMap =  {
  red: Colors.RED4,
  blue: Colors.BLUE4,
  yellow: Colors.GOLD4,
  green: Colors.GREEN4,
  orange: Colors.ORANGE1,
}

const defaultColor = Colors.GRAY5;

const colorByTag = d => {
  const color = colorMap[d.tags.color];
  return color || defaultColor;
}

function indexOfNearestValueSorted(values, val) {
  let min = 0;
  let max = values.length - 1;
  let mid = Math.floor((min + max) / 2);
  let midVal = values[mid];

  while (min < max) {
    if (midVal < val) {
      min = mid + 1;
    } else if (midVal > val) {
      max = mid - 1;
    } else {
      return mid;
    }

    mid = Math.floor((min + max) / 2);
    midVal = values[mid];
  }

  if (midVal < val) {
    return mid + 1;
  } else if (midVal > val) {
    return mid;
  } else {
    return mid;
  }
}

const colorByNumericTag = (values, colorMin, colorMax, tagName) => {
  const colorMinArray = colorMin.match(/[a-fA-F0-9]{2}/g).map(x => parseInt(x, 16));
  const colorMaxArray = colorMax.match(/[a-fA-F0-9]{2}/g).map(x => parseInt(x, 16));

  return  d => {
    console.log({ d, values })
    if (typeof d.tags[tagName] !== "number") {
      return defaultColor;
    }

    const value = indexOfNearestValueSorted(values, d.tags[tagName]);
    const colorArray = colorMinArray.map((x, i) => {
      const y = colorMaxArray[i];
      return Math.round(x + (y - x) * value / values.length);
    });

    return  "#" + colorArray.map(x => x.toString(16).padStart(2, "0")).join("");
  }
}

export const pickColoringStrategy = (dataset, datapoints) => {
  let colorBy = dataset?.metadata?.colorable?.[0]?.name;
  if (!colorBy) {
    return colorByTag;
  } else {
    const values = datapoints.map(d => d.tags[colorBy]);
    values.sort((a, b) => a - b);

    return colorByNumericTag(values, Colors.BLUE3, Colors.RED3, colorBy);
  }
}

