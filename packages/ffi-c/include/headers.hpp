#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Buffer {
  uint8_t *data;
  int len;
};

extern "C" {

Buffer invoke(const char *client_ptr, const char *uri, const char *method, const char *args);

const char *create_client(const char *resolver_ptr);

const char *create_resolver();

} // extern "C"
