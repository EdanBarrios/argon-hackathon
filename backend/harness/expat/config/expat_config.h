/*
 * Hand-written expat_config.h for the Theseus harness (macOS / Linux, clang).
 * Replaces the CMake-generated file so build_expat.sh needs no cmake.
 * Mirrors a default CMake build of libexpat R_2_9_0: XML_DTD, XML_GE=1,
 * XML_NS compiled in (the driver never enables namespace processing),
 * XML_CONTEXT_BYTES 1024.
 */
#ifndef EXPAT_CONFIG_H
#define EXPAT_CONFIG_H 1

#if defined(__BYTE_ORDER__) && __BYTE_ORDER__ == __ORDER_BIG_ENDIAN__
#  define BYTEORDER 4321
#  define WORDS_BIGENDIAN 1
#else
#  define BYTEORDER 1234
#endif

#define HAVE_FCNTL_H 1
#define HAVE_INTTYPES_H 1
#define HAVE_STDINT_H 1
#define HAVE_STDLIB_H 1
#define HAVE_STRING_H 1
#define HAVE_STRINGS_H 1
#define HAVE_SYS_STAT_H 1
#define HAVE_SYS_TYPES_H 1
#define HAVE_UNISTD_H 1
#define HAVE_GETPAGESIZE 1
#define HAVE_MMAP 1
#define STDC_HEADERS 1

/* Entropy source for the hash salt. The driver sets a fixed salt with
   XML_SetHashSalt16Bytes, so this is never consulted at run time, but the
   library still needs one source compiled in. */
#if defined(__APPLE__)
#  define HAVE_ARC4RANDOM_BUF 1
#endif
#define XML_DEV_URANDOM 1

#define PACKAGE "expat"
#define PACKAGE_BUGREPORT "https://github.com/libexpat/libexpat/issues"
#define PACKAGE_NAME "expat"
#define PACKAGE_STRING "expat 2.9.0"
#define PACKAGE_TARNAME "expat"
#define PACKAGE_URL ""
#define PACKAGE_VERSION "2.9.0"

#define XML_CONTEXT_BYTES 1024
#define XML_DTD 1
#define XML_GE 1
#define XML_NS 1

#endif /* EXPAT_CONFIG_H */
