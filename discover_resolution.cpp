#include "discover_resolution.h"
#include <stdint.h>
#include <dirent.h>
#include <string.h>
#include <vector>
#include <string>

#define DEFAULT_WIDTH (2448)
#define DEFAULT_HEIGHT (1224)

struct Edid {
  char id[4];
  uint8_t product_code[2];
  int width;
  int height;
};

Edid parse_edid(std::vector<uint8_t> data) {
  Edid out;
  memset(&out, 0, sizeof(Edid));
  if (data.size() < 128) {
    return out;
  }

  out.id[0] = ((data[8] & 0x7C) >> 2) + '@';
	out.id[1] = ((data[8] & 0x03) << 3) + ((data[9] & 0xE0) >> 5) + '@';
	out.id[2] = (data[9] & 0x1F) + '@';
  out.product_code[0] = data[10];
  out.product_code[1] = data[11];
  uint8_t num_ext = data[126];
  int off = 128;
  if (num_ext < 1 || data.size() < 256 || data[off] != 0x70) {
    // we expect first extension to be displayid one
    return out;
  }
  off++;
  if (data[off] != 0x20) {
    // display id 2.0 expected
    return out;
  }
  off += 4;
  if (data[off] == 0x22) {
    // Video Timing Modes Type 7 - Detailed Timings Data Block
    off += 3;
    auto w0 = data[off+4];
    auto w1 = data[off+5];
    auto h0 = data[off+12];
    auto h1 = data[off+13];
    out.width = 1 + (w1 << 8 | w0);
    out.height = 1 + (h1 << 8 | h0);
  }

  return out;
}

std::vector<uint8_t> read_file(const char *path) {
  auto f = fopen(path, "r");
  if (f == nullptr) {
    return {};
  }

  std::vector<uint8_t> v;
  v.resize(64 * 1024);
  auto n = fread(&v[0], 1, 64*1024, f);
  v.resize(n);
  fclose(f);
  return v;
}

Resolution discover_resolution() {
  DIR *d = opendir("/sys/class/drm");
  dirent *e;
  Resolution out = {DEFAULT_WIDTH, DEFAULT_HEIGHT};
  while (e = readdir(d)) {
    auto p = std::string("/sys/class/drm/") + e->d_name + "/edid";
    auto data = read_file(p.c_str());
    if (data.size() > 0) {
      Edid edid = parse_edid(data);
      if (strcmp(edid.id, "HVR") == 0 && edid.product_code[0] == 0x04 && edid.product_code[1] == 0xAA && edid.width > 0 && edid.height > 0) {
        out = Resolution{edid.width, edid.height};
        break;
      }
    }
  }
  closedir(d);
  return out;
}
