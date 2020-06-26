#include <sys/sdt.h>
#include "tide-trace.h"

void startroute(char* method, char* path, int id)   
{
    DTRACE_PROBE3(tide, startroute, method, path, id);
}

void endroute(char* method, char* path, int id, int status, char* headers)
{
    DTRACE_PROBE5(tide, endroute, method, path, id, status, headers);
}

void probe(char* tag, char* data) {
    DTRACE_PROBE2(tide, startroute, tag, data);
}
