/*
 * Differential-test driver for cJSON parse + print.
 *
 * Reads all of stdin, parses it with cJSON_ParseWithLengthOpts
 * (require_null_terminated = 1), and prints exactly one line:
 *   PARSE_ERROR at <offset>    when parsing fails
 *   OK\t<unformatted JSON>     when parsing succeeds
 * Always exits 0 unless the process crashes or a sanitizer aborts it.
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
        if (len + 1 >= cap) { /* keep one spare byte for the NUL */
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
    cJSON *root;
    char *printed;

    if (input == NULL) {
        fputs("driver: failed to read stdin\n", stderr);
        return 2;
    }

    /*
     * With require_null_terminated = 1, cJSON requires a NUL byte INSIDE the
     * given length right after the value (plus optional whitespace). So we
     * append one and pass len + 1, the same convention cJSON_ParseWithOpts
     * uses (strlen + sizeof("")). Without this every input is rejected.
     * Consequence: an embedded NUL ends the document, exactly as it does for
     * cJSON_Parse on a C string.
     */
    input[len] = '\0';
    root = cJSON_ParseWithLengthOpts((const char *)input, len + 1, &end, 1);
    if (root == NULL) {
        const char *err = cJSON_GetErrorPtr();
        if (err != NULL && err >= (const char *)input && err <= (const char *)input + len) {
            printf("PARSE_ERROR at %ld\n", (long)(err - (const char *)input));
        } else {
            printf("PARSE_ERROR\n");
        }
        free(input);
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
    free(input);
    return 0;
}
