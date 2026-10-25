/* Stub expat-ABI shim: exports every symbol in contracts/pyexpat-shim.md so the
 * Python side can be built and loaded before the Rust shim exists. It parses
 * nothing: XML_Parse fails with XML_ERROR_SYNTAX (2).
 * build: clang -dynamiclib -I<expat lib dir> -DXML_DTD -DXML_NS -DXML_GE=1 \
 *   -install_name @rpath/libtheseus_expat_stub.dylib stub_shim.c -o libtheseus_expat_stub.dylib */
#include <stdlib.h>
#include <string.h>
#include "expat.h"

struct XML_ParserStruct {
  void *user_data;
  enum XML_Error err;
  char *buf;
  int buflen;
};

#define SETTER(name, type) \
  void XMLCALL name(XML_Parser p, type h) { (void)p; (void)h; }

XML_Parser XMLCALL XML_ParserCreate_MM(const XML_Char *e, const XML_Memory_Handling_Suite *m,
                                       const XML_Char *s) {
  (void)e; (void)m; (void)s;
  return calloc(1, sizeof(struct XML_ParserStruct));
}
XML_Parser XMLCALL XML_ExternalEntityParserCreate(XML_Parser p, const XML_Char *c, const XML_Char *e) {
  (void)p; (void)c; (void)e; return NULL;
}
void XMLCALL XML_ParserFree(XML_Parser p) { if (p) { free(p->buf); free(p); } }
enum XML_Status XMLCALL XML_Parse(XML_Parser p, const char *s, int len, int fin) {
  (void)s; (void)len; (void)fin; p->err = XML_ERROR_SYNTAX; return XML_STATUS_ERROR;
}
void *XMLCALL XML_GetBuffer(XML_Parser p, int len) {
  if (len > p->buflen) { free(p->buf); p->buf = malloc(len ? len : 1); p->buflen = len; }
  return p->buf;
}
enum XML_Status XMLCALL XML_ParseBuffer(XML_Parser p, int len, int fin) {
  return XML_Parse(p, p->buf, len, fin);
}
enum XML_Status XMLCALL XML_StopParser(XML_Parser p, XML_Bool r) { (void)p; (void)r; return XML_STATUS_ERROR; }
enum XML_Error XMLCALL XML_GetErrorCode(XML_Parser p) { return p->err; }
const XML_LChar *XMLCALL XML_ErrorString(enum XML_Error c) {
  if (c == XML_ERROR_NONE) return NULL;
  if ((int)c > 0 && (int)c <= XML_ERROR_NOT_STARTED) return "stub error";
  return NULL;
}
const XML_LChar *XMLCALL XML_ExpatVersion(void) { return "expat_stub"; }
XML_Expat_Version XMLCALL XML_ExpatVersionInfo(void) {
  XML_Expat_Version v = {XML_MAJOR_VERSION, XML_MINOR_VERSION, XML_MICRO_VERSION};
  return v;
}
const XML_Feature *XMLCALL XML_GetFeatureList(void) {
  static const XML_Feature f[] = {{XML_FEATURE_END, NULL, 0}};
  return f;
}
XML_Size XMLCALL XML_GetCurrentLineNumber(XML_Parser p) { (void)p; return 1; }
XML_Size XMLCALL XML_GetCurrentColumnNumber(XML_Parser p) { (void)p; return 0; }
XML_Index XMLCALL XML_GetCurrentByteIndex(XML_Parser p) { (void)p; return 0; }
const char *XMLCALL XML_GetInputContext(XML_Parser p, int *o, int *s) { (void)p; (void)o; (void)s; return NULL; }
int XMLCALL XML_GetSpecifiedAttributeCount(XML_Parser p) { (void)p; return 0; }
const XML_Char *XMLCALL XML_GetBase(XML_Parser p) { (void)p; return NULL; }
enum XML_Status XMLCALL XML_SetBase(XML_Parser p, const XML_Char *b) { (void)p; (void)b; return XML_STATUS_OK; }
enum XML_Status XMLCALL XML_SetEncoding(XML_Parser p, const XML_Char *e) { (void)p; (void)e; return XML_STATUS_OK; }
void XMLCALL XML_FreeContentModel(XML_Parser p, XML_Content *m) { (void)p; free(m); }
void XMLCALL XML_SetUserData(XML_Parser p, void *u) { p->user_data = u; }
int XMLCALL XML_SetHashSalt(XML_Parser p, unsigned long s) { (void)p; (void)s; return 1; }
int XMLCALL XML_SetParamEntityParsing(XML_Parser p, enum XML_ParamEntityParsing x) { (void)p; (void)x; return 1; }
enum XML_Error XMLCALL XML_UseForeignDTD(XML_Parser p, XML_Bool b) { (void)p; (void)b; return XML_ERROR_NONE; }
XML_Bool XMLCALL XML_SetReparseDeferralEnabled(XML_Parser p, XML_Bool e) { (void)p; (void)e; return XML_TRUE; }
void XMLCALL XML_SetReturnNSTriplet(XML_Parser p, int d) { (void)p; (void)d; }
void XMLCALL XML_SetElementHandler(XML_Parser p, XML_StartElementHandler s, XML_EndElementHandler e) { (void)p; (void)s; (void)e; }
void XMLCALL XML_SetNamespaceDeclHandler(XML_Parser p, XML_StartNamespaceDeclHandler s, XML_EndNamespaceDeclHandler e) { (void)p; (void)s; (void)e; }
void XMLCALL XML_SetUnknownEncodingHandler(XML_Parser p, XML_UnknownEncodingHandler h, void *d) { (void)p; (void)h; (void)d; }
SETTER(XML_SetAttlistDeclHandler, XML_AttlistDeclHandler)
SETTER(XML_SetCharacterDataHandler, XML_CharacterDataHandler)
SETTER(XML_SetCommentHandler, XML_CommentHandler)
SETTER(XML_SetDefaultHandler, XML_DefaultHandler)
SETTER(XML_SetDefaultHandlerExpand, XML_DefaultHandler)
SETTER(XML_SetElementDeclHandler, XML_ElementDeclHandler)
SETTER(XML_SetEndCdataSectionHandler, XML_EndCdataSectionHandler)
SETTER(XML_SetEndDoctypeDeclHandler, XML_EndDoctypeDeclHandler)
SETTER(XML_SetEndElementHandler, XML_EndElementHandler)
SETTER(XML_SetEndNamespaceDeclHandler, XML_EndNamespaceDeclHandler)
SETTER(XML_SetEntityDeclHandler, XML_EntityDeclHandler)
SETTER(XML_SetExternalEntityRefHandler, XML_ExternalEntityRefHandler)
SETTER(XML_SetNotStandaloneHandler, XML_NotStandaloneHandler)
SETTER(XML_SetNotationDeclHandler, XML_NotationDeclHandler)
SETTER(XML_SetProcessingInstructionHandler, XML_ProcessingInstructionHandler)
SETTER(XML_SetSkippedEntityHandler, XML_SkippedEntityHandler)
SETTER(XML_SetStartCdataSectionHandler, XML_StartCdataSectionHandler)
SETTER(XML_SetStartDoctypeDeclHandler, XML_StartDoctypeDeclHandler)
SETTER(XML_SetStartElementHandler, XML_StartElementHandler)
SETTER(XML_SetStartNamespaceDeclHandler, XML_StartNamespaceDeclHandler)
SETTER(XML_SetUnparsedEntityDeclHandler, XML_UnparsedEntityDeclHandler)
SETTER(XML_SetXmlDeclHandler, XML_XmlDeclHandler)
