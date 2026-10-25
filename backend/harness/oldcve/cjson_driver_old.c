/*
 * CVE demo driver for an OLD, vulnerable cJSON (v1.7.17).
 *
 * Same stdin/stdout protocol as backend/harness/drivers/cjson_driver.c:
 *   PARSE_ERROR at <offset>    when parsing fails
 *   OK\t<unformatted JSON>     when parsing succeeds
 * Always exits 0 unless the process crashes or a sanitizer aborts it.
 *
 * ONE deliberate difference from the 1.7.19 driver, required to reach
 * CVE-2023-53154 (heap buffer over-read in the object/string parse path):
 * we call cJSON_ParseWithLengthOpts on a buffer of EXACTLY len bytes with
 * require_null_terminated = 0, and we do NOT append a trailing NUL. The
 * 1.7.19 driver appends a NUL and passes len + 1, which gives the parser a
 * readable sentinel byte and so hides this bug even on a vulnerable library.
 * The CVE is specifically about cJSON_ParseWithLength on a buffer that is not
 * NUL-terminated at the given length, so this is the faithful trigger path.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "cJSON.h"

static unsigned char *read_stdin(size_t *out_len)
{
    size_t cap = 4096;
    size_t len = 0;
    unsigned char *buf = (unsigned char *)malloc(cap);
    if (buf == NULL) {
        return NULL;
    }
    for (;;) {
        size_t n;
        if (len >= cap) {
            unsigned char *grown;
            cap *= 2;
            grown = (unsigned char *)realloc(buf, cap);
            if (grown == NULL) {
                free(buf);
                return NULL;
            }
            buf = grown;
        }
        n = fread(buf + len, 1, cap - len, stdin);
        len += n;
        if (n == 0) {
            break;
        }
    }
    if (ferror(stdin)) {
        free(buf);
        return NULL;
    }
    *out_len = len;
    return buf;
}

int main(void)
{
    size_t len = 0;
    const char *end = NULL;
    unsigned char *input = read_stdin(&len);
    unsigned char *exact;
    cJSON *root;
    char *printed;

    if (input == NULL) {
        fputs("driver: failed to read stdin\n", stderr);
        return 2;
    }

    /*
     * Reallocate to EXACTLY len bytes so ASan places a redzone immediately
     * after the last input byte. Any read past the stated length is then a
     * hard heap-buffer-overflow, which is the whole point of the demo.
     */
    exact = (unsigned char *)malloc(len == 0 ? 1 : len);
    if (exact == NULL) {
        fputs("driver: oom\n", stderr);
        free(input);
        return 2;
    }
    if (len > 0) {
        memcpy(exact, input, len);
    }
    free(input);

    /* require_null_terminated = 0, length = exact len, no appended NUL. */
    root = cJSON_ParseWithLengthOpts((const char *)exact, len, &end, 0);
    if (root == NULL) {
        const char *err = cJSON_GetErrorPtr();
        if (err != NULL && err >= (const char *)exact && err <= (const char *)exact + len) {
            printf("PARSE_ERROR at %ld\n", (long)(err - (const char *)exact));
        } else {
            printf("PARSE_ERROR\n");
        }
        free(exact);
        return 0;
    }

    printed = cJSON_PrintUnformatted(root);
    if (printed == NULL) {
        printf("PRINT_ERROR\n");
    } else {
        fputs("OK\t", stdout);
        fputs(printed, stdout);
        fputc('\n', stdout);
        cJSON_free(printed);
    }
    cJSON_Delete(root);
    free(exact);
    return 0;
}
