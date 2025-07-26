#pragma once

#define assert(condition) ((void)0)

#define RAND_MAX 2147483647
#define FLT_MAX  0.0f

#define stdout 0

typedef unsigned char  uint8_t;
typedef short          int16_t;
typedef unsigned short uint16_t;
typedef int            int32_t;
typedef unsigned int   uint32_t;
typedef unsigned long  size_t;

extern "C" void          spine_wasm_todo      ();
extern "C" void*         spine_wasm_malloc    (size_t s);
extern "C" void*         spine_wasm_realloc   (void* p, size_t s);
extern "C" void          spine_wasm_free      (void* p);
extern "C" bool          spine_wasm_isnan     (double a);
extern "C" int           spine_wasm_isspace   (int c);
extern "C" int           spine_wasm_abs       (int n);
extern "C" double        spine_wasm_fmod      (double x, double y);
extern "C" double        spine_wasm_atan2     (double y, double x);
extern "C" double        spine_wasm_cos       (double a);
extern "C" double        spine_wasm_sin       (double a);
extern "C" double        spine_wasm_sqrt      (double a);
extern "C" double        spine_wasm_acos      (double a);
extern "C" double        spine_wasm_nan       (const char* a);
extern "C" double        spine_wasm_pow       (double b, double e);
extern "C" double        spine_wasm_ceil      (double a);
extern "C" int           spine_wasm_rand      ();
extern "C" int           spine_wasm_strcmp    (const char* l, const char* r);
extern "C" size_t        spine_wasm_strlen    (const char* s);
extern "C" void*         spine_wasm_memcpy    (void* d, const void* s, size_t c);
extern "C" void*         spine_wasm_memset    (void* s, int c, size_t n);
extern "C" char*         spine_wasm_strrchr   (const char* s, int c);
extern "C" char*         spine_wasm_strdup    (const char* s);
extern "C" long          spine_wasm_strtol    (const char* s, char** e, int b);
extern "C" unsigned long spine_wasm_strtoul   (const char* s, char** e, int b);
extern "C" char*         spine_wasm_strcpy    (char* d, const char* s);
extern "C" char*         spine_wasm_strncat   (char* d, const char* s, size_t c);
extern "C" int           spine_wasm_strncmp   (const char* l, const char* r, size_t c);
extern "C" int           spine_wasm_strcasecmp(const char* s1, const char* s2);
extern "C" int           spine_wasm_fflush    (void* s);

static inline void*         malloc    (size_t s)                               { return spine_wasm_malloc(s);          }
static inline void*         realloc   (void* p, size_t s)                      { return spine_wasm_realloc(p, s);      }
static inline void          free      (void* p)                                { return spine_wasm_free(p);            }
static inline bool          isnan     (double a)                               { return spine_wasm_isnan(a);           }
static inline int           isspace   (int c)                                  { return spine_wasm_isspace(c);         }
static inline int           abs       (int n)                                  { return spine_wasm_abs(n);             }
static inline double        fmod      (double x, double y)                     { return spine_wasm_fmod(x, y);         }
static inline double        atan2     (double x, double y)                     { return spine_wasm_atan2(x, y);        }
static inline double        cos       (double a)                               { return spine_wasm_cos(a);             }
static inline double        sin       (double a)                               { return spine_wasm_sin(a);             }
static inline double        sqrt      (double a)                               { return spine_wasm_sqrt(a);            }
static inline double        acos      (double a)                               { return spine_wasm_acos(a);            }
static inline double        nan       (const char* a)                          { return spine_wasm_nan(a);             }
static inline double        pow       (double b, double e)                     { return spine_wasm_pow(b, e);          }
static inline double        ceil      (double a)                               { return spine_wasm_ceil(a);            }
static inline int           rand      ()                                       { return spine_wasm_rand();             }
static inline int           strcmp    (const char* l, const char* r)           { return spine_wasm_strcmp(l, r);       }
static inline size_t        strlen    (const char* s)                          { return spine_wasm_strlen(s);          }
static inline void*         memcpy    (void* d, const void* s, size_t c)       { return spine_wasm_memcpy(d, s, c);    }
static inline void*         memset    (void* s, int c, size_t n)               { return spine_wasm_memset(s, c, n);    }
static inline char*         strrchr   (const char* s, int c)                   { return spine_wasm_strrchr(s, c);      }
static inline char*         strdup    (const char* s)                          { return spine_wasm_strdup(s);          }
static inline long          strtol    (const char* s, char** e, int b)         { return spine_wasm_strtol(s, e, b);    }
static inline unsigned long strtoul   (const char* s, char** e, int b)         { return spine_wasm_strtoul(s, e, b);   }
static inline char*         strcpy    (char* d, const char* s)                 { return spine_wasm_strcpy(d, s);       }
static inline char*         strncat   (char* d, const char* s, size_t c)       { return spine_wasm_strncat(d, s, c);   }
static inline int           strncmp   (const char* l, const char* r, size_t c) { return spine_wasm_strncmp(l, r, c);   }
static inline int           strcasecmp(const char* s1, const char* s2)         { return spine_wasm_strcasecmp(s1, s2); }
static inline int           fflush    (void* s)                                { return spine_wasm_fflush(s);          }

namespace std {
    static inline bool   isnan(double a)      { return spine_wasm_isnan(a); }
    static inline double nan  (const char* a) { return spine_wasm_nan(a);   }
}

static inline int printf  (const char * format, ...)                    { spine_wasm_todo(); return 0; } // TODO
static inline int snprintf(char* s, size_t n, const char* format, ...)  { spine_wasm_todo(); return 0; } // TODO
static inline int scanf   (const char* format, ...)                     { spine_wasm_todo(); return 0; } // TODO
static inline int sscanf  (const char* buffer, const char* format, ...) { spine_wasm_todo(); return 0; } // TODO

void* operator new(size_t size, void* ptr) noexcept;
