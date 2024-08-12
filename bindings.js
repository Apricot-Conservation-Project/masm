export function render_map(x) {
  return URL.createObjectURL(new Blob([render_map_(new Uint8Array(x))]));
}

export function render_schem(x) {
  return URL.createObjectURL(new Blob([render_schem_(new Uint8Array(x))]));
}

export function tags(x) {
  let map = {};
  tags_(new Uint8Array(x)).forEach(
    (tag) => (map[tag.split("⬟")[0]] = tag.split("⬟")[1])
  );
  return map;
}
