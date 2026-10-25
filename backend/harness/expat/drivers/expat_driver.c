/*
 * Differential-test driver for libexpat (Theseus harness, role C).
 *
 * Reads all of stdin, creates a parser with XML_ParserCreate(NULL) (encoding
 * from the BOM or XML declaration, UTF-8 otherwise), sets a fixed all-zero
 * hash salt, and calls XML_Parse once with isFinal = 1. Every callback
 * appends one event line to an in-memory buffer. Then:
 *   success: print all event lines, then the line OK
 *   failure: print ONLY the line ERROR <code> line <L> col <C>
 *            (events seen before the error are discarded)
 * where <code> is XML_GetErrorCode (an enum XML_Error value) and L / C come
 * from XML_GetCurrentLineNumber64 / XML_GetCurrentColumnNumber64.
 * Always exits 0 unless the process crashes or a sanitizer aborts it.
 *
 * Scope: no namespace processing (XML_ParserCreate, not ...NS), no external
 * entity loading (no external entity ref handler, parameter entity parsing
 * left at the default XML_PARAM_ENTITY_PARSING_NEVER), internal DTD entities
 * expanded, billion-laughs protection at library defaults.
 *
 * Event lines (one per callback, in callback order):
 *   XMLDECL <version> <encoding> <standalone>
 *   DOCTYPE_START <name> <sysid> <pubid> <has_internal_subset>
 *   DOCTYPE_END
 *   ENTITY <name> <is_pe> VALUE <value>
 *   ENTITY <name> <is_pe> EXTERNAL <sysid> <pubid> <notation>
 *   START <name>[ <attr>=<value>]...   attributes in the order expat gives
 *                                       them (specified first, then defaulted)
 *   END <name>
 *   TEXT <text>                          adjacent character data coalesced
 *   PI <target> <data>
 *   COMMENT <text>
 *   CDATA_START
 *   CDATA_END
 *   SKIPPED <name> <is_pe>
 * Every string field is printed as a quoted string ("...") or as the bare
 * word null when expat passes NULL. Inside quotes, bytes 0x20..0x7E print as
 * themselves except '"' and '\\', which print as \" and \\; every other byte
 * prints as \xNN (two lowercase hex digits). Integers print in decimal.
 * Pending TEXT is flushed before any other event and before the final line.
 */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "expat.h"

/* ---- output buffer ----------------------------------------------------- */
/* All event lines go to this buffer. It is written to stdout only if the
   parse succeeds; on error only the ERROR line is printed. This matches the
   Rust interface parse(&[u8]) -> Result<Vec<Event>, XmlError>. */

static char *out_buf = NULL;
static size_t out_len = 0;
static size_t out_cap = 0;

static void out_bytes(const char *s, size_t n)
{
    if (out_len + n > out_cap) {
        size_t cap = out_cap ? out_cap : 65536;
        char *grown;
        while (cap < out_len + n) {
            cap *= 2;
        }
        grown = (char *)realloc(out_buf, cap);
        if (grown == NULL) {
            fputs("driver: out of memory\n", stderr);
            exit(2);
        }
        out_buf = grown;
        out_cap = cap;
    }
    memcpy(out_buf + out_len, s, n);
    out_len += n;
}

static void out_char(char c)
{
    out_bytes(&c, 1);
}

static void out_cstr(const char *s)
{
    out_bytes(s, strlen(s));
}

static void out_int(int v)
{
    char tmp[32];
    int n = snprintf(tmp, sizeof tmp, "%d", v);
    out_bytes(tmp, (size_t)n);
}

/* ---- output helpers ---------------------------------------------------- */

static void put_quoted_n(const char *s, size_t n)
{
    static const char hex[] = "0123456789abcdef";
    size_t i;
    out_char('"');
    for (i = 0; i < n; i++) {
        unsigned char c = (unsigned char)s[i];
        if (c == '"' || c == '\\') {
            out_char('\\');
            out_char(c);
        } else if (c >= 0x20 && c <= 0x7e) {
            out_char(c);
        } else {
            out_char('\\');
            out_char('x');
            out_char(hex[c >> 4]);
            out_char(hex[c & 15]);
        }
    }
    out_char('"');
}

static void put_str(const XML_Char *s)
{
    if (s == NULL) {
        out_cstr("null");
    } else {
        put_quoted_n(s, strlen(s));
    }
}

/* ---- character data coalescing ----------------------------------------- */

static char *text_buf = NULL;
static size_t text_len = 0;
static size_t text_cap = 0;
static int text_pending = 0;

static void flush_text(void)
{
    if (!text_pending) {
        return;
    }
    out_cstr("TEXT ");
    put_quoted_n(text_buf, text_len);
    out_char('\n');
    text_len = 0;
    text_pending = 0;
}

static void XMLCALL on_text(void *ud, const XML_Char *s, int len)
{
    (void)ud;
    if (len <= 0) { /* empty runs never produce a TEXT line */
        return;
    }
    if (text_len + (size_t)len > text_cap) {
        size_t cap = text_cap ? text_cap : 4096;
        char *grown;
        while (cap < text_len + (size_t)len) {
            cap *= 2;
        }
        grown = (char *)realloc(text_buf, cap);
        if (grown == NULL) {
            fputs("driver: out of memory\n", stderr);
            exit(2);
        }
        text_buf = grown;
        text_cap = cap;
    }
    memcpy(text_buf + text_len, s, (size_t)len);
    text_len += (size_t)len;
    text_pending = 1;
}

/* ---- other callbacks ---------------------------------------------------- */

static void XMLCALL on_xmldecl(void *ud, const XML_Char *version,
                               const XML_Char *encoding, int standalone)
{
    (void)ud;
    flush_text();
    out_cstr("XMLDECL ");
    put_str(version);
    out_char(' ');
    put_str(encoding);
    out_char(' '); out_int(standalone); out_char('\n');
}

static void XMLCALL on_doctype_start(void *ud, const XML_Char *name,
                                     const XML_Char *sysid, const XML_Char *pubid,
                                     int has_internal_subset)
{
    (void)ud;
    flush_text();
    out_cstr("DOCTYPE_START ");
    put_str(name);
    out_char(' ');
    put_str(sysid);
    out_char(' ');
    put_str(pubid);
    out_char(' '); out_int(has_internal_subset); out_char('\n');
}

static void XMLCALL on_doctype_end(void *ud)
{
    (void)ud;
    flush_text();
    out_cstr("DOCTYPE_END\n");
}

static void XMLCALL on_entity_decl(void *ud, const XML_Char *name, int is_pe,
                                   const XML_Char *value, int value_length,
                                   const XML_Char *base, const XML_Char *sysid,
                                   const XML_Char *pubid, const XML_Char *notation)
{
    (void)ud;
    (void)base; /* never set: the driver does not call XML_SetBase */
    flush_text();
    out_cstr("ENTITY ");
    put_str(name);
    out_char(' '); out_int(is_pe); out_char(' ');
    if (value != NULL) {
        out_cstr("VALUE ");
        put_quoted_n(value, value_length > 0 ? (size_t)value_length : 0);
    } else {
        out_cstr("EXTERNAL ");
        put_str(sysid);
        out_char(' ');
        put_str(pubid);
        out_char(' ');
        put_str(notation);
    }
    out_char('\n');
}

static void XMLCALL on_start(void *ud, const XML_Char *name, const XML_Char **atts)
{
    int i;
    (void)ud;
    flush_text();
    out_cstr("START ");
    put_str(name);
    for (i = 0; atts[i] != NULL; i += 2) {
        out_char(' ');
        put_str(atts[i]);
        out_char('=');
        put_str(atts[i + 1]);
    }
    out_char('\n');
}

static void XMLCALL on_end(void *ud, const XML_Char *name)
{
    (void)ud;
    flush_text();
    out_cstr("END ");
    put_str(name);
    out_char('\n');
}

static void XMLCALL on_pi(void *ud, const XML_Char *target, const XML_Char *data)
{
    (void)ud;
    flush_text();
    out_cstr("PI ");
    put_str(target);
    out_char(' ');
    put_str(data);
    out_char('\n');
}

static void XMLCALL on_comment(void *ud, const XML_Char *data)
{
    (void)ud;
    flush_text();
    out_cstr("COMMENT ");
    put_str(data);
    out_char('\n');
}

static void XMLCALL on_cdata_start(void *ud)
{
    (void)ud;
    flush_text();
    out_cstr("CDATA_START\n");
}

static void XMLCALL on_cdata_end(void *ud)
{
    (void)ud;
    flush_text();
    out_cstr("CDATA_END\n");
}

static void XMLCALL on_skipped(void *ud, const XML_Char *name, int is_pe)
{
    (void)ud;
    flush_text();
    out_cstr("SKIPPED ");
    put_str(name);
    out_char(' '); out_int(is_pe); out_char('\n');
}

/* ---- main --------------------------------------------------------------- */

static char *read_stdin(size_t *out_len)
{
    size_t cap = 65536;
    size_t len = 0;
    char *buf = (char *)malloc(cap);
    if (buf == NULL) {
        return NULL;
    }
    for (;;) {
        size_t n;
        if (len == cap) {
            char *grown;
            cap *= 2;
            grown = (char *)realloc(buf, cap);
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
    static const uint8_t zero_salt[16] = {0};
    size_t len = 0;
    char *input = read_stdin(&len);
    XML_Parser parser;
    enum XML_Status status;

    if (input == NULL || len > (size_t)0x7fffffff) {
        fputs("driver: failed to read stdin\n", stderr);
        return 2;
    }

    parser = XML_ParserCreate(NULL);
    if (parser == NULL) {
        fputs("driver: XML_ParserCreate failed\n", stderr);
        return 2;
    }
    /* Fixed salt: no entropy read, identical hashing on every run. */
    XML_SetHashSalt16Bytes(parser, zero_salt);

    XML_SetXmlDeclHandler(parser, on_xmldecl);
    XML_SetDoctypeDeclHandler(parser, on_doctype_start, on_doctype_end);
    XML_SetEntityDeclHandler(parser, on_entity_decl);
    XML_SetElementHandler(parser, on_start, on_end);
    XML_SetCharacterDataHandler(parser, on_text);
    XML_SetProcessingInstructionHandler(parser, on_pi);
    XML_SetCommentHandler(parser, on_comment);
    XML_SetCdataSectionHandler(parser, on_cdata_start, on_cdata_end);
    XML_SetSkippedEntityHandler(parser, on_skipped);

    status = XML_Parse(parser, input, (int)len, 1);
    flush_text();
    if (status == XML_STATUS_OK) {
        out_cstr("OK\n");
        fwrite(out_buf, 1, out_len, stdout);
    } else {
        /* Events seen before the error are discarded on purpose. */
        printf("ERROR %d line %llu col %llu\n", (int)XML_GetErrorCode(parser),
               (unsigned long long)XML_GetCurrentLineNumber64(parser),
               (unsigned long long)XML_GetCurrentColumnNumber64(parser));
    }
    fflush(stdout);

    XML_ParserFree(parser);
    free(text_buf);
    free(out_buf);
    free(input);
    return 0;
}
