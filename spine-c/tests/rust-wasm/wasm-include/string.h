typedef unsigned long size_t;
namespace std {
    typedef unsigned long size_t;
}
int strcmp( const char* lhs, const char* rhs );
size_t strlen( const char* str );
void* memcpy( void *dest, const void *src, size_t count );
void *memset(void *str, int c, size_t n);
char* strrchr( const char* str, int ch );
long      strtol( const char*          str, char**          str_end, int base );
char* strcpy( char* dest, const char* src );
char* strncat( char* dest, const char* src, std::size_t count );
int strncmp( const char* lhs, const char* rhs, std::size_t count );
int strcasecmp(const char *s1, const char *s2);
char * strdup( const char *str1 );
