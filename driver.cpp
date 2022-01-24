#include "driver.h"

typedef void *(*HmdDriverFactory_ty)(const char *pInterfaceName,
                                     int *pReturnCode);
HmdDriverFactory_ty HmdDriverFactoryReal;

static int lens_server_in;
static int lens_server_out;

int read_exact(int fd, void *buf, size_t nbytes) {
  while (nbytes > 0) {
    int res = read(fd, buf, nbytes);
    if (res == -1)
      return -1;
    else if (res == 0)
      return -1;
    nbytes -= res;
    buf = (void *)((size_t)buf + res);
  }
  return 0;
}

static pthread_mutex_t cs_mutex = PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP;

class CVRDisplayComponent : public vr::IVRDisplayComponent {
public:
  CVRDisplayComponent(vr::IVRDisplayComponent *_real) : real(_real) {}
  vr::IVRDisplayComponent *real;
  virtual void GetWindowBounds(int32_t *pnX, int32_t *pnY, uint32_t *pnWidth,
                               uint32_t *pnHeight) {
    DEBUG("GetWindowBounds()\n");
    real->GetWindowBounds(pnX, pnY, pnWidth, pnHeight);
    DEBUG("original driver returned %d %d %dx%d\n", *pnX, *pnY, *pnWidth,
          *pnHeight);
    *pnX = 0;
    *pnY = 0;
    *pnWidth = 2448;
    *pnHeight = 1224;
    DEBUG("proxy driver returned %d %d %dx%d\n", *pnX, *pnY, *pnWidth,
          *pnHeight);
  }
  virtual bool IsDisplayOnDesktop() { return real->IsDisplayOnDesktop(); }
  virtual bool IsDisplayRealDisplay() { return real->IsDisplayRealDisplay(); }
  virtual void GetRecommendedRenderTargetSize(uint32_t *pnWidth,
                                              uint32_t *pnHeight) {
    real->GetRecommendedRenderTargetSize(pnWidth, pnHeight);
  }
  virtual void GetEyeOutputViewport(vr::EVREye eEye, uint32_t *pnX,
                                    uint32_t *pnY, uint32_t *pnWidth,
                                    uint32_t *pnHeight) {
    DEBUG("GetEyeOutputViewport(%d)\n", eEye);
    real->GetEyeOutputViewport(eEye, pnX, pnY, pnWidth, pnHeight);
    DEBUG("original driver returned %d %d %dx%d\n", *pnX, *pnY, *pnWidth,
          *pnHeight);
    *pnWidth = 2448 / 2;
    *pnHeight = 1224;
    *pnY = 0;
    *pnX = eEye == vr::Eye_Left ? 0 : *pnWidth;
  }
  virtual void GetProjectionRaw(vr::EVREye eEye, float *pfLeft, float *pfRight,
                                float *pfTop, float *pfBottom) {
    pthread_mutex_lock(&cs_mutex);
    ServerInputProjectionRaw input = {1, eEye};
    ASSERT_FATAL(
        write(lens_server_in, &input, sizeof(ServerInputProjectionRaw)) != -1,
        "write");
    ServerOutputProjectionRaw output;
    ASSERT_FATAL(read_exact(lens_server_out, &output,
                            sizeof(ServerOutputProjectionRaw)) != -1,
                 "read");
    pthread_mutex_unlock(&cs_mutex);
    *pfLeft = output.left;
    *pfRight = output.right;
    *pfTop = output.top;
    *pfBottom = output.bottom;
    DEBUG("LRTB: %f %f %f %f\n", *pfLeft, *pfRight, *pfTop, *pfBottom);
  }
  virtual vr::DistortionCoordinates_t ComputeDistortion(vr::EVREye eEye,
                                                        float fU, float fV) {
    pthread_mutex_lock(&cs_mutex);
    ServerInputDistort input = {0, eEye, fU, INVERT_MATRIX ? 1.0f - fV : fV};
    ASSERT_FATAL(write(lens_server_in, &input, sizeof(ServerInputDistort)) !=
                     -1,
                 "write");
    ServerOutputDistort output;
    ASSERT_FATAL(
        read_exact(lens_server_out, &output, sizeof(ServerOutputDistort)) != -1,
        "read");
    pthread_mutex_unlock(&cs_mutex);
    return {{output.red[0], output.red[1]},
            {output.green[0], output.green[1]},
            {output.blue[0], output.blue[1]}};
  }
};

class CVRHmdDriver final : public vr::ITrackedDeviceServerDriver {
public:
  CVRHmdDriver(vr::ITrackedDeviceServerDriver *_real) : real(_real) {}
  vr::ITrackedDeviceServerDriver *real;
  virtual vr::EVRInitError Activate(uint32_t unObjectId) {
    return real->Activate(unObjectId);
  }
  virtual void Deactivate() { real->Deactivate(); }
  virtual void EnterStandby() { real->EnterStandby(); }
  virtual void *GetComponent(const char *pchComponentNameAndVersion) {
    DEBUG("GetComponent(%s)\n", pchComponentNameAndVersion);
    void *result = real->GetComponent(pchComponentNameAndVersion);
    if (strcmp(pchComponentNameAndVersion, vr::IVRDisplayComponent_Version) ==
        0) {
      result = (vr::IVRDisplayComponent *)new CVRDisplayComponent(
          (vr::IVRDisplayComponent *)result);
    }
    return result;
  }
  virtual void DebugRequest(const char *pchRequest, char *pchResponseBuffer,
                            uint32_t unResponseBufferSize) {
    real->DebugRequest(pchRequest, pchResponseBuffer, unResponseBufferSize);
  }
  virtual vr::DriverPose_t GetPose() { return real->GetPose(); }
};

class CVRServerDriverHost : public vr::IVRServerDriverHost {
public:
  CVRServerDriverHost(vr::IVRServerDriverHost *_real) : real(_real) {}
  vr::IVRServerDriverHost *real;

  virtual bool TrackedDeviceAdded(const char *pchDeviceSerialNumber,
                                  vr::ETrackedDeviceClass eDeviceClass,
                                  vr::ITrackedDeviceServerDriver *pDriver) {
    DEBUG("TrackedDeviceAdded(%s, %d)\n", pchDeviceSerialNumber, eDeviceClass);
    if (eDeviceClass == vr::TrackedDeviceClass_HMD) {
      pDriver = new CVRHmdDriver(pDriver);
    }
    return real->TrackedDeviceAdded(pchDeviceSerialNumber, eDeviceClass,
                                    pDriver);
  }
  virtual void TrackedDevicePoseUpdated(uint32_t unWhichDevice,
                                        const vr::DriverPose_t &newPose,
                                        uint32_t unPoseStructSize) {
    real->TrackedDevicePoseUpdated(unWhichDevice, newPose, unPoseStructSize);
  }
  virtual void VsyncEvent(double vsyncTimeOffsetSeconds) {
    real->VsyncEvent(vsyncTimeOffsetSeconds);
  }
  virtual void VendorSpecificEvent(uint32_t unWhichDevice,
                                   vr::EVREventType eventType,
                                   const vr::VREvent_Data_t &eventData,
                                   double eventTimeOffset) {
    real->VendorSpecificEvent(unWhichDevice, eventType, eventData,
                              eventTimeOffset);
  }
  virtual bool IsExiting() { return real->IsExiting(); }
  virtual bool PollNextEvent(vr::VREvent_t *pEvent, uint32_t uncbVREvent) {
    return real->PollNextEvent(pEvent, uncbVREvent);
  }
  virtual void
  GetRawTrackedDevicePoses(float fPredictedSecondsFromNow,
                           vr::TrackedDevicePose_t *pTrackedDevicePoseArray,
                           uint32_t unTrackedDevicePoseArrayCount) {
    real->GetRawTrackedDevicePoses(fPredictedSecondsFromNow,
                                   pTrackedDevicePoseArray,
                                   unTrackedDevicePoseArrayCount);
  }
  virtual void RequestRestart(const char *pchLocalizedReason,
                              const char *pchExecutableToStart,
                              const char *pchArguments,
                              const char *pchWorkingDirectory) {
    real->RequestRestart(pchLocalizedReason, pchExecutableToStart, pchArguments,
                         pchWorkingDirectory);
  }
  virtual uint32_t GetFrameTimings(vr::Compositor_FrameTiming *pTiming,
                                   uint32_t nFrames) {
    return real->GetFrameTimings(pTiming, nFrames);
  }
  virtual void SetDisplayEyeToHead(uint32_t unWhichDevice,
                                   const vr::HmdMatrix34_t &eyeToHeadLeft,
                                   const vr::HmdMatrix34_t &eyeToHeadRight) {
    real->SetDisplayEyeToHead(unWhichDevice, eyeToHeadLeft, eyeToHeadRight);
  }
  virtual void SetDisplayProjectionRaw(uint32_t unWhichDevice,
                                       const vr::HmdRect2_t &eyeLeft,
                                       const vr::HmdRect2_t &eyeRight) {
    real->SetDisplayProjectionRaw(unWhichDevice, eyeLeft, eyeRight);
  }
  virtual void SetRecommendedRenderTargetSize(uint32_t unWhichDevice,
                                              uint32_t nWidth,
                                              uint32_t nHeight) {
    real->SetRecommendedRenderTargetSize(unWhichDevice, nWidth, nHeight);
  }
};

class CVRDriverContext : public vr::IVRDriverContext {
public:
  CVRDriverContext(vr::IVRDriverContext *_real) : real(_real) {}
  vr::IVRDriverContext *real;

  virtual void *GetGenericInterface(const char *pchInterfaceVersion,
                                    vr::EVRInitError *peError = nullptr) {
    DEBUG("GetGenericInterface(%s)\n", pchInterfaceVersion);
    void *result = real->GetGenericInterface(pchInterfaceVersion, peError);

    if (strcmp(pchInterfaceVersion, vr::IVRServerDriverHost_Version) == 0) {
      result = new CVRServerDriverHost((vr::IVRServerDriverHost *)result);
    }

    return result;
  }
  virtual vr::DriverHandle_t GetDriverHandle() {
    DEBUG("GetDriverHandle()\n");
    return real->GetDriverHandle();
  }
};

class CServerTrackedDeviceProvider : public vr::IServerTrackedDeviceProvider {
public:
  CServerTrackedDeviceProvider(vr::IServerTrackedDeviceProvider *_real)
      : real(_real) {}
  vr::IServerTrackedDeviceProvider *real;

  virtual vr::EVRInitError Init(vr::IVRDriverContext *pDriverContext) {
    DEBUG("Init()\n");
    return real->Init(new CVRDriverContext(pDriverContext));
  }
  virtual void Cleanup() {
    DEBUG("Cleanup()\n");
    real->Cleanup();
  }
  virtual const char *const *GetInterfaceVersions() {
    DEBUG("GetInterfaceVersions()\n");
    auto result = real->GetInterfaceVersions();
    auto it = result;
    DEBUG("interfaces:\n");
    for (auto iface = *it; iface; iface = *++it) {
      DEBUG("- %s\n", iface);
    }
    DEBUG("done\n");
    return result;
  }
  virtual void RunFrame() { real->RunFrame(); }
  virtual bool ShouldBlockStandbyMode() {
    DEBUG("ShouldBlockStandbyMode()\n");
    return real->ShouldBlockStandbyMode();
  }
  virtual void EnterStandby() {
    DEBUG("EnterStandby()\n");
    real->EnterStandby();
  }
  virtual void LeaveStandby() {
    DEBUG("LeaveStandby()\n");
    real->LeaveStandby();
  }
};
static CServerTrackedDeviceProvider *server_device_provider;

extern "C" __attribute__((visibility("default"))) void *
HmdDriverFactory(const char *pInterfaceName, int *pReturnCode) {
  if (HmdDriverFactoryReal == nullptr) {
    DEBUG("loading library from %s\n", LIGHTHOUSE_BIN);
    void *library =
        dlopen(LIGHTHOUSE_BIN "/driver_lighthouse_real.so", RTLD_NOW);
    ASSERT_RET(library != nullptr, "library not found");
    HmdDriverFactoryReal =
        (HmdDriverFactory_ty)dlsym(library, "HmdDriverFactory");
    ASSERT_RET(HmdDriverFactoryReal != nullptr, "symbol not found");
    DEBUG("initialized real factory");
  }
  if (lens_server_out == 0) {
    DEBUG("starting lens server from %s", LENS_SERVER_DIST);
    int inpipefd[2];
    int outpipefd[2];
    ASSERT_RET(pipe(inpipefd) == 0, "pipe failed");
    ASSERT_RET(pipe(outpipefd) == 0, "pipe failed");
    if (fork() == 0) {
      DEBUG("hello from lens process");
      dup2(inpipefd[0], STDIN_FILENO);
      dup2(outpipefd[1], STDOUT_FILENO);
      prctl(PR_SET_PDEATHSIG, SIGTERM);

      DEBUG("executing [%s %s]", WINE, LENS_SERVER_DIST "/lens-server.exe");
      const char *env[] = {"WINEDEBUG=-all", NULL};
      execle(WINE, "wine", LENS_SERVER_DIST "/lens-server.exe", (char *)nullptr,
             env);
      exit(1);
    }

    DEBUG("testing lens server\n");
    ServerInputDistort input = {0, 0, 0.0, 0.0};
    ASSERT_RET(write(inpipefd[1], &input, sizeof(ServerInputDistort)) != -1,
               "write failed");

    ServerOutputDistort output;
    ASSERT_RET(read_exact(outpipefd[0], &output, sizeof(ServerOutputDistort)) !=
                   -1,
               "read failed");
    DEBUG("request completed, assuming lens server is fine\n");
    lens_server_in = inpipefd[1];
    lens_server_out = outpipefd[0];
  }

  if (strcmp(pInterfaceName, vr::IServerTrackedDeviceProvider_Version) == 0) {
    if (server_device_provider == nullptr) {
      DEBUG("server tracked init\n");
      auto real = (vr::IServerTrackedDeviceProvider *)HmdDriverFactoryReal(
          pInterfaceName, pReturnCode);
      server_device_provider = new CServerTrackedDeviceProvider(real);
    }
    if (pReturnCode) {
      *pReturnCode = vr::VRInitError_None;
    }
    return server_device_provider;
  }

  DEBUG("requested unknown interface = %s\n", pInterfaceName);
  if (pReturnCode) {
    *pReturnCode = vr::VRInitError_Init_InterfaceNotFound;
  }
  return nullptr;
}
