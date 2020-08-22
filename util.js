export function gen_rand(range_start, range_end) {
  return (
    Math.floor(Math.random() * (range_end - range_start) + 1) + range_start
  );
}
