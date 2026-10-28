/* Minimal trace-pc-guard runtime: each guard gets an index; on exit, write the
 * hit/not-hit byte per guard to $EDGE_OUT (if set). */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
static uint8_t *hits;
static uint32_t nguards;
static void dump(void) {
  const char *p = getenv("EDGE_OUT");
  if (!p || !hits) return;
  FILE *f = fopen(p, "wb");
  if (!f) return;
  fwrite(hits, 1, nguards, f);
  fclose(f);
}
void __sanitizer_cov_trace_pc_guard_init(uint32_t *start, uint32_t *stop) {
  if (start == stop || *start) return;
  uint32_t n = 0;
  for (uint32_t *x = start; x < stop; x++) *x = ++n;
  nguards = n;
  hits = calloc(n + 1, 1);
  atexit(dump);
}
void __sanitizer_cov_trace_pc_guard(uint32_t *guard) {
  if (*guard) hits[*guard - 1] = 1;
}
