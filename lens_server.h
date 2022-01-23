struct ServerInputDistort {
  char id;
  char eye;
  float u;
  float v;
} __attribute__((__packed__));
struct ServerOutputDistort {
  float red[2];
  float green[2];
  float blue[2];
} __attribute__((__packed__));

struct ServerInputProjectionRaw {
  char id;
  char eye;
} __attribute__((__packed__));
struct ServerOutputProjectionRaw {
  float left;
  float right;
  float top;
  float bottom;
} __attribute__((__packed__));
