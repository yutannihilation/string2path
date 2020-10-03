#include <stdint.h>

#ifdef __cplusplus
extern "C"
{
#endif

    // A struct to pass the results from Rust to C
    typedef struct
    {
        double *x;
        double *y;
        uint32_t *id;
        uint32_t *glyph_id;
        uint32_t length;
    } Result;

    void free_result(Result);

    Result string2path(const char *, const char *, double, double, uint32_t);

#ifdef __cplusplus
}
#endif
