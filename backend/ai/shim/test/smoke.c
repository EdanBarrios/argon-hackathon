/* Smoke test for libtheseus_expat: links against expat.h 2.9.0 like pyexpat does.
 * Build + run: backend/ai/shim/test/run.sh. Prints one line per check, exits 1 on a failure. */
#include <stdio.h>
#include <string.h>
#include "expat.h"

static int fails = 0;
#define CHECK(name, cond) do { int ok_ = (cond); printf("%s %s\n", ok_ ? "PASS" : "FAIL", name); if (!ok_) fails++; } while (0)

static char log_[4096];
static void add(const char *s) { strncat(log_, s, sizeof(log_) - strlen(log_) - 1); }

static void XMLCALL on_start(void *ud, const XML_Char *name, const XML_Char **atts) {
  (void)ud; add("<"); add(name);
  for (int i = 0; atts[i]; i += 2) { add(" "); add(atts[i]); add("="); add(atts[i + 1]); }
  add(">");
}
static void XMLCALL on_end(void *ud, const XML_Char *name) { (void)ud; add("</"); add(name); add(">"); }
static void XMLCALL on_text(void *ud, const XML_Char *s, int len) {
  (void)ud; char b[256]; int n = len < 255 ? len : 255; memcpy(b, s, n); b[n] = 0; add("["); add(b); add("]");
}
static void XMLCALL on_comment(void *ud, const XML_Char *s) { (void)ud; add("<!--"); add(s); add("-->"); }
static void XMLCALL stop_on_start(void *ud, const XML_Char *name, const XML_Char **atts) {
  (void)atts; add("<"); add(name); add(">"); XML_StopParser((XML_Parser)ud, XML_FALSE);
}
static void XMLCALL clear_on_start(void *ud, const XML_Char *name, const XML_Char **atts) {
  (void)atts; add("<"); add(name); add(">"); XML_SetElementHandler((XML_Parser)ud, NULL, NULL);
}

int main(void) {
  /* 1. basic events through split XML_Parse calls */
  log_[0] = 0;
  XML_Parser p = XML_ParserCreate(NULL);
  XML_SetElementHandler(p, on_start, on_end);
  XML_SetCharacterDataHandler(p, on_text);
  XML_SetCommentHandler(p, on_comment);
  const char *doc = "<a x=\"1\">hi<!--c--><b/></a>";
  int r1 = XML_Parse(p, doc, 5, 0);
  int r2 = XML_Parse(p, doc + 5, (int)strlen(doc) - 5, 1);
  printf("  events: %s\n", log_);
  CHECK("parse ok", r1 == XML_STATUS_OK && r2 == XML_STATUS_OK);
  CHECK("events", strcmp(log_, "<a x=1>[hi]<!--c--><b></b></a>") == 0);
  CHECK("error code none", XML_GetErrorCode(p) == XML_ERROR_NONE);
  XML_ParserFree(p);

  /* 2. error state */
  p = XML_ParserCreate(NULL);
  int r = XML_Parse(p, "<a>\n<b></a>", 11, 1);
  printf("  error: %d %s line %lu col %lu\n", XML_GetErrorCode(p), XML_ErrorString(XML_GetErrorCode(p)),
         (unsigned long)XML_GetCurrentLineNumber(p), (unsigned long)XML_GetCurrentColumnNumber(p));
  CHECK("mismatch is error", r == XML_STATUS_ERROR && XML_GetErrorCode(p) == XML_ERROR_TAG_MISMATCH);
  CHECK("error position", XML_GetCurrentLineNumber(p) == 2 && XML_GetCurrentColumnNumber(p) == 5);
  XML_ParserFree(p);

  /* 3. StopParser from a handler aborts the replay */
  log_[0] = 0;
  p = XML_ParserCreate(NULL);
  XML_SetUserData(p, p);
  XML_SetElementHandler(p, stop_on_start, on_end);
  r = XML_Parse(p, "<a><b/></a>", 11, 1);
  CHECK("stop -> aborted", r == XML_STATUS_ERROR && XML_GetErrorCode(p) == XML_ERROR_ABORTED && strcmp(log_, "<a>") == 0);
  XML_ParserFree(p);

  /* 4. handler cleared mid-parse is not called again */
  log_[0] = 0;
  p = XML_ParserCreate(NULL);
  XML_SetUserData(p, p);
  XML_SetElementHandler(p, clear_on_start, on_end);
  r = XML_Parse(p, "<a><b/></a>", 11, 1);
  CHECK("cleared handler not called", r == XML_STATUS_OK && strcmp(log_, "<a>") == 0);
  XML_ParserFree(p);

  /* 5. namespace parser: plain doc ok, xmlns doc is a clean error */
  log_[0] = 0;
  p = XML_ParserCreate_MM(NULL, NULL, "}");
  XML_SetElementHandler(p, on_start, on_end);
  r = XML_Parse(p, "<a><b/></a>", 11, 1);
  CHECK("ns parser, no namespaces", r == XML_STATUS_OK && strcmp(log_, "<a><b></b></a>") == 0);
  XML_ParserFree(p);
  p = XML_ParserCreate_MM(NULL, NULL, "}");
  r = XML_Parse(p, "<a xmlns=\"u\"/>", 14, 1);
  CHECK("ns parser, xmlns -> error", r == XML_STATUS_ERROR && XML_GetErrorCode(p) != XML_ERROR_NONE);
  XML_ParserFree(p);

  /* 6. GetBuffer / ParseBuffer */
  log_[0] = 0;
  p = XML_ParserCreate(NULL);
  XML_SetElementHandler(p, on_start, on_end);
  void *buf = XML_GetBuffer(p, 64);
  memcpy(buf, "<r/>", 4);
  r = XML_ParseBuffer(p, 4, 1);
  CHECK("GetBuffer/ParseBuffer", buf && r == XML_STATUS_OK && strcmp(log_, "<r></r>") == 0);
  XML_ParserFree(p);

  /* 7. version, error strings, features */
  XML_Expat_Version v = XML_ExpatVersionInfo();
  printf("  version: %s\n", XML_ExpatVersion());
  CHECK("version", v.major == 2 && v.minor == 9 && v.micro == 0 && strstr(XML_ExpatVersion(), "theseus"));
  int all = 1;
  for (int c = 1; c <= XML_ERROR_NOT_STARTED; c++) if (!XML_ErrorString((enum XML_Error)c)) all = 0;
  CHECK("error strings 1..44", all);
  const XML_Feature *f = XML_GetFeatureList();
  int n = 0; while (f && f[n].feature != XML_FEATURE_END) n++;
  CHECK("feature list terminated", f && n > 0);

  printf("%s\n", fails ? "SMOKE FAILED" : "SMOKE OK");
  return fails ? 1 : 0;
}
