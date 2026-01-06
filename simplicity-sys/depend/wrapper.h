#ifndef WRAPPER_H
#define WRAPPER_H

#include "simplicity/frame.h"
#include "simplicity/sha256.h"

#define WRAP_(jet) \
bool rustsimplicity_0_6_c_##jet(frameItem* dst, const frameItem* src, const txEnv* env) { \
  bool result = rustsimplicity_0_6_##jet(dst, *src, env); \
  rustsimplicity_0_6_assert(!result || 0 == dst->offset); \
  return result; \
}

/* Write a 256-bit hash value to the 'dst' frame, advancing the cursor 256 cells.
 *
 * Precondition: '*dst' is a valid write frame for 256 more cells;
 *               NULL != h;
 */
void rustsimplicity_0_6_writeHash(frameItem* dst, const sha256_midstate* h);

#endif
