#include "lens_server.h"
#include "openvr_driver.h"
#include <csignal>
#include <cstring>
#include <dlfcn.h>
#include <sys/prctl.h>
#include <unistd.h>

#define DEBUG(fmt, ...)                                                        \
  do {                                                                         \
    fprintf(stderr, "PROXY %s(%d): " fmt "\n", __func__, __LINE__,             \
            ##__VA_ARGS__);                                                    \
    fflush(stderr);                                                            \
  } while (0)

#define ASSERT_RET(condition, fmt, ...)                                        \
  do {                                                                         \
    if (!(condition)) {                                                        \
      DEBUG("assert failed (%s): " fmt, #condition, ##__VA_ARGS__);            \
      if (pReturnCode) {                                                       \
        *pReturnCode = vr::VRInitError_Init_Internal;                          \
      }                                                                        \
      return nullptr;                                                          \
    }                                                                          \
  } while (0)

#define ASSERT_FATAL(condition, fmt, ...)                                      \
  do {                                                                         \
    if (!(condition)) {                                                        \
      DEBUG("fatal assert failed (%s): " fmt, #condition, ##__VA_ARGS__);      \
      exit(1);                                                                 \
    }                                                                          \
  } while (0)
