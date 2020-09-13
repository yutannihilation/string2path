#include <stdint.h>

#ifdef __cplusplus
extern "C"
{
#endif

    typedef struct
    {
        double *data;
        uint32_t length;
    } Result;

    int32_t glyph2digit(const char *, const char *);
    Result string2path(const char *, const char *);

#ifdef __cplusplus
}
#endif
